use crate::{crypto, NoteGenerator, ObfuscatedNote, SecretKey};

#[test]
fn decrypt() {
    let sk = SecretKey::default();
    let pk = sk.public_key();
    let vk = sk.view_key();

    let (r, r_g, _) = ObfuscatedNote::generate_pk_r(&pk);

    let bytes = b"some data";
    let encrypted = crypto::encrypt(&r, &pk, bytes);
    let decrypted = crypto::decrypt(&r_g, &vk, encrypted.as_slice());

    assert_eq!(&bytes[..], decrypted.as_slice());
}

#[test]
fn decrypt_with_wrong_key_should_fail() {
    let sk = SecretKey::default();
    let pk = sk.public_key();
    let vk = sk.view_key();
    let wrong_vk = SecretKey::default().view_key();

    assert_ne!(vk, wrong_vk);

    let (r, r_g, _) = ObfuscatedNote::generate_pk_r(&pk);

    let bytes = b"some data";
    let encrypted = crypto::encrypt(&r, &pk, bytes);
    let decrypted = crypto::decrypt(&r_g, &wrong_vk, encrypted.as_slice());

    assert_ne!(&bytes[..], decrypted.as_slice());
}

#[test]
fn decrypt_obfuscated_note() {
    let value = 25;

    let sk = SecretKey::default();
    let pk = sk.public_key();
    let vk = sk.view_key();

    let note = ObfuscatedNote::output(&pk, value);
    let decrypt_value = note.decrypt_value(&vk);

    assert_eq!(decrypt_value, value);
}
