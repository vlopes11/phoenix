use crate::{
    crypto, rpc, utils, zk, BlsScalar, Error, Note, NoteGenerator, ObfuscatedNote, PublicKey,
    SecretKey, TransparentNote,
};

use std::convert::TryFrom;
use std::io::{self, Read, Write};
use std::path::Path;
use std::{fmt, ptr};

use num_traits::Zero;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

pub const MAX_INPUT_NOTES_PER_TRANSACTION: usize = 1;
pub const MAX_OUTPUT_NOTES_PER_TRANSACTION: usize = 2;

/// Maximum allowed number of notes per transaction.
pub const MAX_NOTES_PER_TRANSACTION: usize =
    MAX_INPUT_NOTES_PER_TRANSACTION + MAX_OUTPUT_NOTES_PER_TRANSACTION;

/// Serialized bytes size
pub const TX_SERIALIZED_SIZE: usize = 1876;

pub use item::{TransactionInput, TransactionItem, TransactionOutput};

lazy_static::lazy_static! {
    static ref DEFAULT_INPUT: TransactionInput = TransactionInput::default();
    static ref DEFAULT_OUTPUT: TransactionOutput = TransactionOutput::default();
}

/// Transaction item definitions
pub mod item;

/// A phoenix transaction
#[derive(Clone)]
pub struct Transaction {
    fee: TransactionOutput,
    idx_inputs: usize,
    inputs: [TransactionInput; MAX_INPUT_NOTES_PER_TRANSACTION],
    idx_outputs: usize,
    outputs: [TransactionOutput; MAX_OUTPUT_NOTES_PER_TRANSACTION],
    proof: Option<zk::Proof>,
    public_inputs: zk::ZkPublicInputs,
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            fee: *DEFAULT_OUTPUT,
            idx_inputs: 0,
            inputs: [*DEFAULT_INPUT; MAX_INPUT_NOTES_PER_TRANSACTION],
            idx_outputs: 0,
            outputs: [*DEFAULT_OUTPUT; MAX_OUTPUT_NOTES_PER_TRANSACTION],
            proof: None,
            public_inputs: zk::ZkPublicInputs::default(),
        }
    }
}

impl Read for Transaction {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        self.clear_sensitive_info();

        let mut n = 0;

        let proof = self
            .proof
            .as_ref()
            .map(zk::proof_to_bytes)
            .unwrap_or(Ok([0x00u8; zk::SERIALIZED_PROOF_SIZE]))
            .map_err::<io::Error, _>(|e| e.into())?;
        let b = (&proof[..]).read(buf)?;
        n += b;
        buf = &mut buf[b..];

        let b = self.public_inputs.read(buf)?;
        n += b;
        buf = &mut buf[b..];

        let inputs = self.idx_inputs.to_le_bytes();
        let b = (&inputs[..]).read(buf)?;
        n += b;
        buf = &mut buf[b..];

        for i in 0..MAX_INPUT_NOTES_PER_TRANSACTION {
            let b = self.inputs[i].read(buf)?;
            n += b;
            buf = &mut buf[b..];
        }

        let outputs = self.idx_outputs.to_le_bytes();
        let b = (&outputs[..]).read(buf)?;
        n += b;
        buf = &mut buf[b..];

        for i in 0..MAX_OUTPUT_NOTES_PER_TRANSACTION {
            let b = self.outputs[i].read(buf)?;
            n += b;
            buf = &mut buf[b..];
        }

        let b = self.fee.read(buf)?;
        n += b;

        Ok(n)
    }
}

impl Write for Transaction {
    fn write(&mut self, mut buf: &[u8]) -> io::Result<usize> {
        let mut n = 0;

        let mut proof = [0x00u8; zk::SERIALIZED_PROOF_SIZE];
        let b = (&mut proof[..]).write(buf)?;
        let proof = zk::bytes_to_proof(&proof[..]).map_err::<io::Error, _>(|e| e.into())?;
        self.proof.replace(proof);
        n += b;
        buf = &buf[b..];

        let b = self.public_inputs.write(buf)?;
        n += b;
        buf = &buf[b..];

        let mut inputs = 0usize.to_le_bytes();
        let b = (&mut inputs[..]).write(buf)?;
        self.idx_inputs = usize::from_le_bytes(inputs);
        n += b;
        buf = &buf[b..];

        for i in 0..MAX_INPUT_NOTES_PER_TRANSACTION {
            let b = self.inputs[i].write(buf)?;
            n += b;
            buf = &buf[b..];
        }

        let mut outputs = 0usize.to_le_bytes();
        let b = (&mut outputs[..]).write(buf)?;
        self.idx_outputs = usize::from_le_bytes(outputs);
        n += b;
        buf = &buf[b..];

        for i in 0..MAX_OUTPUT_NOTES_PER_TRANSACTION {
            let b = self.outputs[i].write(buf)?;
            n += b;
            buf = &buf[b..];
        }

        let b = self.fee.write(buf)?;
        n += b;

        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.public_inputs.flush()?;

        for i in 0..MAX_INPUT_NOTES_PER_TRANSACTION {
            self.inputs[i].flush()?;
        }

        for i in 0..MAX_OUTPUT_NOTES_PER_TRANSACTION {
            self.outputs[i].flush()?;
        }

        self.fee.flush()
    }
}

impl Distribution<Transaction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Transaction {
        let mut input_values = [0u64; MAX_INPUT_NOTES_PER_TRANSACTION];
        let max = u64::max_value() / (MAX_INPUT_NOTES_PER_TRANSACTION as u64) - 1;
        input_values.iter_mut().for_each(|i| {
            *i = rng.gen_range(0, max);
        });
        let inputs: u64 = input_values.iter().sum();

        let mut output_values = [0u64; MAX_OUTPUT_NOTES_PER_TRANSACTION];
        output_values.iter_mut().fold(inputs, |sum, o| {
            *o = rng.gen_range(0, sum);
            sum - *o
        });
        let outputs: u64 = output_values.iter().sum();

        let fee = inputs - outputs;
        debug_assert!(inputs - outputs - fee == 0);

        let mut tx = Transaction::default();

        input_values.iter().for_each(|i| {
            let value = *i;
            if value > 0 {
                let sk = SecretKey::default();
                let pk = sk.public_key();
                let note = TransparentNote::output(&pk, value).0;

                let merkle_opening = crypto::MerkleProof::mock(note.hash());
                tx.push_input(note.to_transaction_input(merkle_opening, sk))
                    .unwrap_or_default();
            }
        });

        output_values.iter().for_each(|o| {
            let value = *o;
            if value > 0 {
                let sk = SecretKey::default();
                let pk = sk.public_key();

                let (note, blinding_factor) = ObfuscatedNote::output(&pk, value);
                tx.push_output(note.to_transaction_output(value, blinding_factor, pk))
                    .unwrap_or_default();
            }
        });

        let sk = SecretKey::default();
        let pk = sk.public_key();
        let (note, blinding_factor) = TransparentNote::output(&pk, fee);
        tx.set_fee(note.to_transaction_output(fee, blinding_factor, pk));

        tx
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.hash().eq(&other.hash())
    }
}
impl Eq for Transaction {}

impl Transaction {
    /// Perform a hash of the inputs, outputs and fee
    pub fn hash(&self) -> BlsScalar {
        // TODO - Maybe improve?

        let mut hash = [BlsScalar::zero(); 2 * MAX_NOTES_PER_TRANSACTION + 1];
        let mut i = 1;

        hash[0] = self.fee.hash();

        let mut items = [TransactionInput::default(); MAX_INPUT_NOTES_PER_TRANSACTION];

        let max_idx = self.idx_inputs;
        if max_idx > 0 {
            items.copy_from_slice(&self.inputs);
            (&mut items[0..max_idx]).sort();

            items[0..max_idx]
                .iter()
                .map(|item| item.note().hash())
                .for_each(|h| {
                    hash[i] = h;
                    i += 1;
                });
        }

        let mut items = [TransactionOutput::default(); MAX_OUTPUT_NOTES_PER_TRANSACTION];

        let max_idx = self.idx_outputs;
        if max_idx > 0 {
            items.copy_from_slice(&self.outputs);
            (&mut items[0..max_idx]).sort();

            items[0..max_idx]
                .iter()
                .map(|item| item.note().hash())
                .for_each(|h| {
                    hash[i] = h;
                    i += 1;
                });
        }

        crypto::sponge_hash(&hash[0..i])
    }

    /// Append an input to the transaction
    pub fn push_input(&mut self, item: TransactionInput) -> Result<(), Error> {
        if self.idx_inputs > MAX_INPUT_NOTES_PER_TRANSACTION {
            return Err(Error::MaximumNotes);
        }

        self.inputs[self.idx_inputs] = item;
        self.idx_inputs += 1;

        Ok(())
    }

    /// Append an output to the transaction
    pub fn push_output(&mut self, item: TransactionOutput) -> Result<(), Error> {
        if self.idx_outputs > MAX_OUTPUT_NOTES_PER_TRANSACTION {
            return Err(Error::MaximumNotes);
        }

        self.outputs[self.idx_outputs] = item;
        self.idx_outputs += 1;

        Ok(())
    }

    /// Return the fee value.
    ///
    /// A transaction is created with a random public key for the fee. The pre-image of the fee
    /// note is not validated on the r1cs circuit, so the public key can later be changed by a
    /// block generator
    pub fn fee(&self) -> &TransactionOutput {
        &self.fee
    }

    /// Set the fee value.
    pub fn set_fee(&mut self, fee: TransactionOutput) {
        self.fee = fee;
    }

    // Set the public key of a block generator. This will not affect the r1cs proof
    pub fn set_fee_pk(&mut self, pk: PublicKey) {
        let value = self.fee.value();
        let (note, blinding_factor) = TransparentNote::output(&pk, value);

        self.fee = note.to_transaction_output(value, blinding_factor, pk);
    }

    /// All transaction inputs, including the dummy non-pushed ones
    pub fn all_inputs(&self) -> &[TransactionInput] {
        &self.inputs[0..MAX_INPUT_NOTES_PER_TRANSACTION]
    }

    /// Transaction inputs
    pub fn inputs(&self) -> &[TransactionInput] {
        &self.inputs[0..self.idx_inputs]
    }

    /// All transaction outputs, including the dummy non-pushed ones
    pub fn all_outputs(&self) -> &[TransactionOutput] {
        &self.outputs[0..MAX_OUTPUT_NOTES_PER_TRANSACTION]
    }

    /// Transaction outputs
    pub fn outputs(&self) -> &[TransactionOutput] {
        &self.outputs[0..self.idx_outputs]
    }

    /// Remove a specified transaction input and return it, if present
    pub fn remove_input(&mut self, idx: usize) -> Option<TransactionInput> {
        if self.idx_inputs == 0 || idx >= self.idx_inputs {
            return None;
        } else if self.idx_inputs == 1 {
            self.idx_inputs = 0;
            return Some(self.inputs[0]);
        }

        self.idx_inputs -= 1;
        let src = (&mut self.inputs[self.idx_inputs]) as *mut TransactionInput;
        let dst = (&mut self.inputs[idx]) as *mut TransactionInput;
        unsafe {
            ptr::swap(src, dst);
        }
        self.inputs[self.idx_inputs] = *DEFAULT_INPUT;

        Some(self.inputs[self.idx_inputs])
    }

    /// Remove a specified transaction output and return it, if present
    pub fn remove_output(&mut self, idx: usize) -> Option<TransactionOutput> {
        if self.idx_outputs == 0 || idx >= self.idx_outputs {
            return None;
        } else if self.idx_outputs == 1 {
            self.idx_outputs = 0;
            return Some(self.outputs[0]);
        }

        self.idx_outputs -= 1;
        let src = (&mut self.outputs[self.idx_outputs]) as *mut TransactionOutput;
        let dst = (&mut self.outputs[idx]) as *mut TransactionOutput;
        unsafe {
            ptr::swap(src, dst);
        }
        self.outputs[self.idx_outputs] = *DEFAULT_OUTPUT;

        Some(self.outputs[self.idx_outputs])
    }

    /// Sort the inputs and outputs
    pub fn sort_items(&mut self) {
        if self.idx_inputs > 0 {
            (&mut self.inputs[0..self.idx_inputs]).sort();
        }

        if self.idx_outputs > 0 {
            (&mut self.outputs[0..self.idx_outputs]).sort();
        }
    }

    pub fn public_inputs(&self) -> &zk::ZkPublicInputs {
        &self.public_inputs
    }

    /// Perform the zk proof, and save internally the created r1cs circuit and the commitment
    /// points.
    ///
    /// Depends on the secret data of the transaction items
    ///
    /// The transaction items will be sorted for verification correctness
    pub fn prove(&mut self) -> Result<(), Error> {
        if self.idx_inputs > MAX_INPUT_NOTES_PER_TRANSACTION
            || self.idx_outputs > MAX_OUTPUT_NOTES_PER_TRANSACTION
        {
            return Err(Error::MaximumNotes);
        }

        self.sort_items();
        let public_inputs = zk::ZkPublicInputs::from(&*self);
        self.public_inputs = public_inputs;

        let proof = zk::prove(self);
        self.proof.replace(proof);

        Ok(())
    }

    /// Return the transaction proof created via [`Transaction::prove`]
    pub fn proof(&self) -> Option<&zk::Proof> {
        self.proof.as_ref()
    }

    /// Replace the current proof, if any
    pub fn set_proof(&mut self, proof: zk::Proof) {
        self.proof.replace(proof);
    }

    /// Remove all the sensitive info from the transaction used to build the zk proof so it can be
    /// safely broadcasted
    pub fn clear_sensitive_info(&mut self) {
        self.inputs
            .iter_mut()
            .for_each(|o| o.clear_sensitive_info());

        self.outputs
            .iter_mut()
            .for_each(|o| o.clear_sensitive_info());
    }

    /// Verify a previously proven transaction with [`Transaction::prove`].
    ///
    /// Doesn't depend on the transaction items secret data. Depends only on the constructed
    /// circuit and commitment points.
    ///
    /// The transaction items will be sorted for verification correctness
    pub fn verify(&self) -> Result<(), Error> {
        let proof = self.proof.as_ref().ok_or(Error::Generic)?;
        let pi = self.public_inputs.generate_pi();

        if zk::verify(proof, pi.as_slice()) {
            Ok(())
        } else {
            Err(Error::Generic)
        }
    }

    /// Create a new transaction from a set of inputs/outputs defined by a rpc source.
    ///
    /// Will prove and verify the created transaction.
    pub fn try_from_rpc_io<P: AsRef<Path>>(
        db_path: P,
        fee_value: u64,
        inputs: &[rpc::TransactionInput],
        outputs: &[rpc::TransactionOutput],
    ) -> Result<Self, Error> {
        let mut transaction = Transaction::default();

        inputs
            .iter()
            .map(|i| {
                TransactionInput::try_from_rpc_transaction_input(db_path.as_ref(), i.clone())
                    .and_then(|i| transaction.push_input(i))
            })
            .collect::<Result<_, _>>()?;

        outputs
            .iter()
            .map(|o| {
                TransactionOutput::try_from(o.clone()).and_then(|o| transaction.push_output(o))
            })
            .collect::<Result<_, _>>()?;

        let pk = PublicKey::default();
        let (fee, blinding_factor) = TransparentNote::output(&pk, fee_value);
        let fee = fee.to_transaction_output(fee_value, blinding_factor, pk);
        transaction.set_fee(fee);

        transaction.prove()?;
        transaction.verify()?;

        Ok(transaction)
    }

    /// Attempt to create a transaction from a rpc request.
    ///
    /// If there is a r1cs proof present on the request, will attempt to verify it against the
    /// proof.
    pub fn try_from_rpc_transaction<P: AsRef<Path>>(
        db_path: P,
        tx: rpc::Transaction,
    ) -> Result<Self, Error> {
        let mut transaction = Transaction::default();

        if let Some(f) = tx.fee {
            transaction.set_fee(TransactionOutput::try_from(f)?);
        }

        tx.inputs
            .iter()
            .map(|i| {
                TransactionInput::try_from_rpc_transaction_input(db_path.as_ref(), i.clone())
                    .and_then(|i| transaction.push_input(i))
            })
            .collect::<Result<_, _>>()?;

        tx.outputs
            .iter()
            .map(|o| {
                TransactionOutput::try_from(o.clone()).and_then(|o| transaction.push_output(o))
            })
            .collect::<Result<_, _>>()?;

        let proof = tx.proof;
        if !proof.is_empty() {
            let proof = zk::bytes_to_proof(proof.as_slice())?;
            transaction.set_proof(proof);
        }

        Ok(transaction)
    }
}

impl TryFrom<Transaction> for rpc::Transaction {
    type Error = Error;

    fn try_from(tx: Transaction) -> Result<rpc::Transaction, Self::Error> {
        let inputs = tx
            .inputs
            .iter()
            .filter_map(|o| {
                if o.value() > 0 {
                    Some((*o).into())
                } else {
                    None
                }
            })
            .collect();

        let outputs = tx
            .outputs
            .iter()
            .filter_map(|o| {
                if o.value() > 0 {
                    Some((*o).into())
                } else {
                    None
                }
            })
            .collect();

        let fee = Some(tx.fee.into());

        let proof = tx
            .proof()
            .map(|p| zk::proof_to_bytes(p).map(|b| b.to_vec()))
            .transpose()?
            .unwrap_or_default();

        // TOD - Serialize and deserialize the pi
        let public_inputs = vec![];

        Ok(rpc::Transaction {
            inputs,
            outputs,
            fee,
            proof,
            public_inputs,
        })
    }
}

impl fmt::LowerHex for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(utils::scalar_as_slice(&self.hash().0)))
    }
}

impl fmt::UpperHex for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            hex::encode_upper(utils::scalar_as_slice(&self.hash().0))
        )
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self)
    }
}
