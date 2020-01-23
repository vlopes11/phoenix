use crate::{
    rpc, Db, Error, Idx, Note, NoteGenerator, NoteType, NoteUtxoType, Nullifier, ObfuscatedNote,
    PublicKey, Scalar, SecretKey, TransparentNote,
};

use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
pub struct TransactionItem {
    note: Box<dyn Note>,
    nullifier: Nullifier,
    value: u64,
    blinding_factor: Scalar,
    sk: Option<SecretKey>,
    pk: PublicKey,
}

impl Clone for TransactionItem {
    fn clone(&self) -> Self {
        TransactionItem {
            note: self.note.box_clone(),
            nullifier: self.nullifier,
            value: self.value,
            blinding_factor: self.blinding_factor,
            sk: None,
            pk: self.pk,
        }
    }
}

impl Default for TransactionItem {
    fn default() -> Self {
        let note = TransparentNote::default();
        let nullifier = Nullifier::default();
        let value = 0;
        let blinding_factor = Scalar::one();
        let sk = Some(SecretKey::default());
        let pk = PublicKey::default();

        TransactionItem::new(note, nullifier, value, blinding_factor, sk, pk)
    }
}

impl TransactionItem {
    pub fn new<N: Note>(
        note: N,
        nullifier: Nullifier,
        value: u64,
        blinding_factor: Scalar,
        sk: Option<SecretKey>,
        pk: PublicKey,
    ) -> Self {
        TransactionItem {
            note: note.box_clone(),
            nullifier,
            value,
            blinding_factor,
            sk,
            pk,
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn idx(&self) -> &Idx {
        self.note.idx()
    }

    pub fn blinding_factor(&self) -> &Scalar {
        &self.blinding_factor
    }

    pub fn note_type(&self) -> NoteType {
        self.note.note()
    }

    pub fn utxo(&self) -> NoteUtxoType {
        self.note.utxo()
    }

    pub fn note(&self) -> Box<dyn Note> {
        self.note.box_clone()
    }

    pub fn nullifier(&self) -> &Nullifier {
        &self.nullifier
    }

    pub fn try_from_rpc_transaction_input(
        db: &Db,
        item: rpc::TransactionInput,
    ) -> Result<Self, Error> {
        let sk: SecretKey = item.sk.ok_or(Error::InvalidParameters).map(|k| k.into())?;
        let note = db.fetch_box_note(&item.pos.ok_or(Error::InvalidParameters)?)?;

        let item = match note.note() {
            NoteType::Transparent => {
                Db::note_box_into::<TransparentNote>(note).to_transaction_input(sk)
            }
            NoteType::Obfuscated => {
                Db::note_box_into::<ObfuscatedNote>(note).to_transaction_input(sk)
            }
        };

        Ok(item)
    }
}

impl TryFrom<rpc::TransactionOutput> for TransactionItem {
    type Error = Error;

    fn try_from(item: rpc::TransactionOutput) -> Result<Self, Self::Error> {
        let pk: PublicKey = item
            .pk
            .ok_or(Error::InvalidParameters)
            .and_then(|k| k.try_into())?;
        let note_type = NoteType::try_from(item.note_type)?;

        match note_type {
            NoteType::Transparent => {
                let (note, blinding_factor) = TransparentNote::output(&pk, item.value);
                Ok(note.to_transaction_output(item.value, blinding_factor, pk))
            }
            NoteType::Obfuscated => {
                let (note, blinding_factor) = ObfuscatedNote::output(&pk, item.value);
                Ok(note.to_transaction_output(item.value, blinding_factor, pk))
            }
        }
    }
}

impl Into<rpc::TransactionInput> for TransactionItem {
    fn into(self) -> rpc::TransactionInput {
        rpc::TransactionInput {
            pos: Some(self.note().idx().clone()),
            sk: self.sk.map(|sk| sk.into()),
        }
    }
}

impl Into<rpc::TransactionOutput> for TransactionItem {
    fn into(self) -> rpc::TransactionOutput {
        rpc::TransactionOutput {
            note_type: self.note().note().into(),
            pk: Some(self.pk.into()),
            value: self.value,
        }
    }
}
