use super::{NoteType, NoteUtxoType, PhoenixIdx, PhoenixNote};
use crate::{
    crypto, hash, utils, CompressedRistretto, Error, PhoenixValue, PublicKey, R1CSProof,
    RistrettoPoint, Scalar, SecretKey,
};
use std::cmp;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
// TODO - Serialization and deserialization should be based on compressed points
pub struct ObfuscatedNote {
    utxo: NoteUtxoType,
    commitments: Vec<CompressedRistretto>,
    r_p: RistrettoPoint,
    pk_r: RistrettoPoint,
    idx: PhoenixIdx,
    encrypted_value: Vec<u8>,
    encrypted_blinding_factors: Vec<u8>,
}

impl ObfuscatedNote {
    pub fn new(
        utxo: NoteUtxoType,
        commitments: Vec<CompressedRistretto>,
        r_p: RistrettoPoint,
        pk_r: RistrettoPoint,
        idx: PhoenixIdx,
        encrypted_value: Vec<u8>,
        encrypted_blinding_factors: Vec<u8>,
    ) -> Self {
        ObfuscatedNote {
            utxo,
            commitments,
            r_p,
            pk_r,
            idx,
            encrypted_value,
            encrypted_blinding_factors,
        }
    }
}

impl ObfuscatedNote {
    fn encrypt_value(pk_r: &RistrettoPoint, value: u64) -> Vec<u8> {
        crypto::encrypt(pk_r, &value.to_le_bytes()[..])
    }

    fn decrypt_value(&self, sk: &SecretKey) -> u64 {
        let decrypt_value = crypto::decrypt(&sk.r_p(&self.r_p), self.encrypted_value.as_slice());

        let mut v = [0x00u8; 8];
        let chunk = cmp::max(decrypt_value.len(), 8);
        (&mut v[0..chunk]).copy_from_slice(&decrypt_value.as_slice()[0..chunk]);

        u64::from_le_bytes(v)
    }

    fn encrypt_blinding_factors(pk_r: &RistrettoPoint, blinding_factors: &[Scalar]) -> Vec<u8> {
        let blinding_factors = blinding_factors
            .iter()
            .map(|s| &s.as_bytes()[..])
            .flatten()
            .map(|b| *b)
            .collect::<Vec<u8>>();

        crypto::encrypt(&pk_r, blinding_factors)
    }

    fn decrypt_blinding_factors(&self, sk: &SecretKey) -> Vec<Scalar> {
        crypto::decrypt(
            &sk.r_p(&self.r_p),
            self.encrypted_blinding_factors.as_slice(),
        )
        .as_slice()
        .chunks(32)
        .map(|b| {
            let mut s = [0x00u8; 32];
            let chunk = cmp::max(b.len(), 32);
            (&mut s[0..chunk]).copy_from_slice(&b[0..chunk]);
            Scalar::from_bits(s)
        })
        .collect()
    }
}

impl PhoenixNote for ObfuscatedNote {
    fn utxo(&self) -> NoteUtxoType {
        self.utxo
    }

    fn note(&self) -> NoteType {
        NoteType::Obfuscated
    }

    fn idx(&self) -> &PhoenixIdx {
        &self.idx
    }

    fn output(pk: &PublicKey, value: u64) -> Self {
        // TODO - Grant r is in Fp
        let r = utils::gen_random_scalar();
        let r_p = utils::scalar_to_field(&r);
        let a_p = pk.a_p;
        let b_p = pk.b_p;
        let pk_r = hash::hash_in_p(&r * &a_p) + b_p;

        let idx = PhoenixIdx::default();
        let phoenix_value = PhoenixValue::new(idx, Scalar::from(value));
        let commitments = phoenix_value.commitments().clone();

        let encrypted_value = ObfuscatedNote::encrypt_value(&pk_r, value);
        let encrypted_blinding_factors =
            ObfuscatedNote::encrypt_blinding_factors(&pk_r, phoenix_value.blinding_factors());

        ObfuscatedNote::new(
            NoteUtxoType::Output,
            commitments,
            r_p,
            pk_r,
            idx,
            encrypted_value,
            encrypted_blinding_factors,
        )
    }

    fn set_idx(mut self, idx: PhoenixIdx) -> Self {
        self.idx = idx;
        self
    }

    fn prove_value(&self, sk_r: &SecretKey) -> Result<R1CSProof, Error> {
        let value = self.decrypt_value(sk_r);
        let blinding_factors = self.decrypt_blinding_factors(sk_r);

        let phoenix_value = PhoenixValue::with_blinding_factors(self.idx, value, blinding_factors);

        phoenix_value.prove(value).map_err(Error::generic)
    }

    fn verify_value(&self, proof: &R1CSProof) -> Result<(), Error> {
        PhoenixValue::with_commitments(self.commitments.clone())
            .verify(proof)
            .map_err(Error::generic)
    }
}
