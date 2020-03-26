use crate::{
    db, rpc, Db, Idx, Note, NoteGenerator, NoteVariant, ObfuscatedNote, PublicKey, Scalar,
    SecretKey, Transaction, TransparentNote, ViewKey,
};

use kelvin::{Blake2b, ByteHash, Root};
use std::convert::TryInto;
use std::env::temp_dir;
use std::fs;

#[test]
fn transaction_items() {
    let (_, _, pk) = generate_keys();
    let value = 25;
    let (note, blinding_factor) = ObfuscatedNote::output(&pk, value);
    let item = note.to_transaction_output(value, blinding_factor, pk);
    assert_eq!(value, item.value());
}

#[test]
fn transaction_zk() {
    // Since we're only working with notes, the db is instantiated here
    // directly and used in the test, as there is no API for directly
    // storing notes without having a `Db` around.
    let mut db = temp_dir();
    db.push("transaction_zk");
    let mut root = Root::<_, Blake2b>::new(db.as_path()).unwrap();
    let mut state: Db<_> = root.restore().unwrap();

    let mut inputs = vec![];
    let mut outputs = vec![];
    let mut senders = vec![];
    let mut receivers = vec![];

    // Setup a sender
    senders.push(generate_keys());
    senders.push(generate_keys());

    // Setup receivers
    receivers.push(generate_keys());
    receivers.push(generate_keys());
    receivers.push(generate_keys());

    // Store an unspent note of 100
    let pk = &senders[0].2;
    inputs.push(create_and_store_unspent_note::<TransparentNote, Blake2b>(
        &mut state, pk, 100,
    ));

    // Store an unspent note of 50
    let pk = &senders[1].2;
    inputs.push(create_and_store_unspent_note::<ObfuscatedNote, Blake2b>(
        &mut state, pk, 50,
    ));

    // Store an unspent note of 45 with the same sender for a malicious proof verification
    let pk = &senders[1].2;
    inputs.push(create_and_store_unspent_note::<ObfuscatedNote, Blake2b>(
        &mut state, pk, 45,
    ));

    // Persist changes to disk
    root.set_root(&mut state).unwrap();

    // Create an output of 97
    let pk = &receivers[0].2;
    outputs.push(create_output_note::<TransparentNote>(pk, 97));

    // Create an output of 30
    let pk = &receivers[1].2;
    outputs.push(create_output_note::<ObfuscatedNote>(pk, 30));

    // Create an output of 20
    let pk = &receivers[2].2;
    outputs.push(create_output_note::<ObfuscatedNote>(pk, 20));

    // Create an output of 92 with the same sender for a malicious proof verification
    let pk = &receivers[0].2;
    outputs.push(create_output_note::<TransparentNote>(pk, 92));

    let mut transaction = Transaction::default();

    // Set the 100 unspent note as the input of the transaction
    let sk = senders[0].0;
    let idx = &inputs[0].2;
    let note: TransparentNote = db::fetch_note(db.as_path(), idx)
        .unwrap()
        .try_into()
        .unwrap();
    transaction.push(note.to_transaction_input(sk));

    // Push the 30 output note to the transaction
    let note: ObfuscatedNote = outputs[1].2.clone().try_into().unwrap();
    let blinding_factor = outputs[1].3;
    let pk = outputs[1].0;
    let value = outputs[1].1;
    transaction.push(note.to_transaction_output(value, blinding_factor, pk));

    // Push the 20 output note to the transaction
    let note: ObfuscatedNote = outputs[2].2.clone().try_into().unwrap();
    let blinding_factor = outputs[2].3;
    let pk = outputs[2].0;
    let value = outputs[2].1;
    transaction.push(note.to_transaction_output(value, blinding_factor, pk));

    let mut malicious_transaction = transaction.clone();

    // Push the 97 output note to the transaction
    let note: TransparentNote = outputs[0].2.clone().try_into().unwrap();
    let blinding_factor = outputs[0].3;
    let pk = outputs[0].0;
    let value = outputs[0].1;
    transaction.push(note.to_transaction_output(value, blinding_factor, pk));

    // Set the 45 unspent note as the input of the malicious transaction
    let sk = senders[1].0;
    let idx = &inputs[2].2;
    let note: ObfuscatedNote = db::fetch_note(db.as_path(), idx)
        .unwrap()
        .try_into()
        .unwrap();
    malicious_transaction.push(note.to_transaction_input(sk));

    // Push the 92 output note to the malicious transaction
    let note: TransparentNote = outputs[3].2.clone().try_into().unwrap();
    let blinding_factor = outputs[3].3;
    let pk = outputs[3].0;
    let value = outputs[3].1;
    malicious_transaction.push(note.to_transaction_output(value, blinding_factor, pk));

    let mut insufficient_inputs_transaction = transaction.clone();

    // Set the 50 unspent note as the input of the transaction
    let sk = senders[1].0;
    let idx = &inputs[1].2;
    let note: ObfuscatedNote = db::fetch_note(db.as_path(), idx)
        .unwrap()
        .try_into()
        .unwrap();
    transaction.push(note.to_transaction_input(sk));

    // Grant only transactions with sufficient inputs can be proven
    assert!(insufficient_inputs_transaction.prove().is_err());

    // Proof and verify the main transaction
    transaction.prove().unwrap();
    transaction.verify().unwrap();

    let proof = transaction.r1cs().cloned().unwrap();
    let commitments = transaction.commitments().clone();

    // The malicious transaction should be consistent by itself
    malicious_transaction.prove().unwrap();
    let malicious_proof = malicious_transaction.r1cs().cloned().unwrap();
    let malicious_commitments = malicious_transaction.commitments().clone();
    malicious_transaction.verify().unwrap();

    // Validate no malicious proof or commitments could be verified successfully
    transaction.set_r1cs(malicious_proof.clone());
    transaction.set_commitments(commitments);
    assert!(transaction.verify().is_err());

    transaction.set_r1cs(proof);
    transaction.set_commitments(malicious_commitments.clone());
    assert!(transaction.verify().is_err());

    transaction.set_r1cs(malicious_proof);
    transaction.set_commitments(malicious_commitments);
    assert!(transaction.verify().is_err());

    // The fee should be the difference between the input and output
    assert_eq!(3, transaction.fee().value());

    // Clean up the db
    fs::remove_dir_all(db.as_path()).expect("could not remove temp db");
}

#[test]
fn transactions_with_transparent_notes() {
    let mut db = temp_dir();
    db.push("transactions_with_transparent_notes");
    let mut root = Root::<_, Blake2b>::new(db.as_path()).unwrap();
    let mut state: Db<_> = root.restore().unwrap();

    let mut notes = vec![];
    let mut outputs = vec![];
    let mut senders = vec![];
    let mut receivers = vec![];
    let mut miner_keys = vec![];

    // Setup a sender
    senders.push(generate_keys());

    // Setup receivers
    receivers.push(generate_keys());
    receivers.push(generate_keys());

    // Store an unspent note
    let pk = &senders[0].2;
    notes.push(create_and_store_unspent_note::<TransparentNote, Blake2b>(
        &mut state, pk, 100,
    ));

    // Persist changes to disk
    root.set_root(&mut state).unwrap();

    let idx = &notes[0].2;
    let note: TransparentNote = db::fetch_note(db.as_path(), idx)
        .unwrap()
        .try_into()
        .unwrap();
    let vk = &senders[0].1;
    assert!(note.is_owned_by(vk));

    // Assert the new unspent note is not nullified
    let sk = &senders[0].0;
    let idx = &notes[0].2;
    let note: TransparentNote = db::fetch_note(db.as_path(), idx)
        .unwrap()
        .try_into()
        .unwrap();
    let nullifier = note.generate_nullifier(sk);
    assert!(db::fetch_nullifier(db.as_path(), &nullifier)
        .unwrap()
        .is_none());

    let mut transaction = Transaction::default();

    // Set the first unspent note as the input of the transaction
    let sk = senders[0].0;
    let idx = &notes[0].2;
    let note: TransparentNote = db::fetch_note(db.as_path(), idx)
        .unwrap()
        .try_into()
        .unwrap();
    transaction.push(note.to_transaction_input(sk));

    // Set the fee cost
    miner_keys.push(generate_keys());
    let fee_cost = transaction.fee().note().value(None);

    // Create two output notes for the transaction
    let pk = &receivers[0].2;
    outputs.push(create_output_note::<TransparentNote>(pk, 25));
    let pk = &receivers[1].2;
    outputs.push(create_output_note::<TransparentNote>(pk, 50 - fee_cost));
    let note: TransparentNote = outputs[0].2.clone().try_into().unwrap();
    let blinding_factor = outputs[0].3;
    let pk = outputs[0].0;
    let value = outputs[0].1;
    transaction.push(note.to_transaction_output(value, blinding_factor, pk));
    let note: TransparentNote = outputs[1].2.clone().try_into().unwrap();
    let blinding_factor = outputs[1].3;
    let pk = outputs[1].0;
    let value = outputs[1].1;
    transaction.push(note.to_transaction_output(value, blinding_factor, pk));

    // Execute the transaction
    transaction.prove().unwrap();
    let unspent_outputs = db::store(db.as_path(), &transaction).unwrap();

    // Assert the spent note is nullified
    let sk = &senders[0].0;
    let idx = &notes[0].2;
    let note: TransparentNote = db::fetch_note(db.as_path(), idx)
        .unwrap()
        .try_into()
        .unwrap();
    let nullifier = note.generate_nullifier(sk);
    assert!(db::fetch_nullifier(db.as_path(), &nullifier)
        .unwrap()
        .is_some());

    // Assert the outputs are not nullified
    unspent_outputs.iter().for_each(|idx| {
        let note: TransparentNote = db::fetch_note(db.as_path(), idx)
            .unwrap()
            .try_into()
            .unwrap();
        let sk = receivers
            .iter()
            .fold(SecretKey::default(), |sk, (r_sk, r_vk, _)| {
                if note.is_owned_by(r_vk) {
                    r_sk.clone()
                } else {
                    sk
                }
            });
        let nullifier = note.generate_nullifier(&sk);
        assert!(db::fetch_nullifier(db.as_path(), &nullifier)
            .unwrap()
            .is_none());
    });

    // Clean up the db
    fs::remove_dir_all(db.as_path()).expect("could not remove temp db");
}

#[test]
fn rpc_transaction() {
    let mut root = Root::<_, Blake2b>::new("/tmp/phoenix-meme").unwrap();
    let mut state: Db<_> = root.restore().unwrap();

    let mut senders = vec![];
    let mut receivers = vec![];

    // Setup senders
    senders.push(generate_keys_from(vec![49, 49, 49, 49]));

    // Setup receivers
    receivers.push(generate_keys_from(vec![48, 48, 48, 48]));
    receivers.push(generate_keys());

    // Store an unspent note of 100
    let pk = senders[0].2;
    let sk = senders[0].0;
    let input = create_and_store_unspent_note::<TransparentNote, Blake2b>(&mut state, &pk, 100);
    let input = input.3.to_transaction_input(sk);
    let nul = input.nullifier();
    println!("sender sk: {}", sk);
    println!("sender vk: {}", senders[0].1);

    // Persist changes to disk
    root.set_root(&mut state).unwrap();

    // Show us the keys!
    println!("{}", receivers[0].0);

    // Now make the transactions. Fucking Victor didn't make this easy for me
    // Create an output of 97
    let pk = &receivers[0].2;
    let output = create_output_note::<TransparentNote>(pk, 97);
    let output = output.2.to_transaction_output(97, output.3, *pk);
    println!("{}", pk);

    let mut tx = Transaction::default();
    tx.push(input);
    tx.push(output);
    tx.prove().unwrap();
    println!("{:?}", tx.items()[1].note());
    println!("{:?}", tx.fee());
}

fn generate_keys() -> (SecretKey, ViewKey, PublicKey) {
    let sk = SecretKey::default();
    let vk = sk.view_key();
    let pk = sk.public_key();

    (sk, vk, pk)
}

fn generate_keys_from(bytes: Vec<u8>) -> (SecretKey, ViewKey, PublicKey) {
    let sk = SecretKey::new_from(bytes);
    let vk = sk.view_key();
    let pk = sk.public_key();

    (sk, vk, pk)
}

fn create_and_store_unspent_note<N, H>(
    db: &mut Db<H>,
    pk: &PublicKey,
    value: u64,
) -> (PublicKey, u64, Idx, NoteVariant, Scalar)
where
    N: Clone + Note + NoteGenerator,
    H: ByteHash,
{
    let (note, blinding_factor) = N::output(pk, value);

    let idx = db.store_unspent_note(note.into()).unwrap();
    let note = db.fetch_note(&idx).unwrap();

    (pk.clone(), value, idx, note, blinding_factor)
}

fn create_output_note<N: Note + NoteGenerator>(
    pk: &PublicKey,
    value: u64,
) -> (PublicKey, u64, NoteVariant, Scalar) {
    let (note, blinding_factor) = N::output(pk, value);

    (pk.clone(), value, note.into(), blinding_factor)
}

fn create_and_store_unspent_rpc_note<N, H>(
    db: &mut Db<H>,
    sk: SecretKey,
    value: u64,
) -> rpc::TransactionInput
where
    N: Clone + Note + NoteGenerator,
    H: ByteHash,
{
    let pk = sk.public_key();
    let idx = db
        .store_unspent_note(N::output(&pk, value).0.into())
        .unwrap();
    let pos = Some(idx);
    let sk = Some(sk.into());

    rpc::TransactionInput { pos, sk }
}

fn create_output_rpc_note<N: Note + NoteGenerator>(
    pk: &PublicKey,
    value: u64,
) -> rpc::TransactionOutput {
    let (note, blinding_factor) = N::output(pk, value);
    let note = Some(note.into());
    let blinding_factor = Some(blinding_factor.into());
    let pk = Some((*pk).into());

    rpc::TransactionOutput {
        note,
        pk,
        value,
        blinding_factor,
    }
}
