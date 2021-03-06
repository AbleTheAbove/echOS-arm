use aes_gcm_siv::aead::consts::U32;
use aes_gcm_siv::aead::heapless::{consts::U128, Vec};
use aes_gcm_siv::aead::{generic_array::GenericArray, AeadInPlace, NewAead};

use crate::serial_log;
use aes_gcm_siv::Aes256GcmSiv;

pub fn test_encrypt() -> &'static str {
    // Not implemented because I'm stupid
    serial_log("Prekey gen");
    let key: GenericArray<u8, U32> = *GenericArray::from_slice(b"an example very very secret key.");

    serial_log("Pre cipher gen");
    let cipher = Aes256GcmSiv::new(&key); // You need a [u8; 16] filled with random bytes

    serial_log("pre nonce");
    let nonce = GenericArray::from_slice(b"unique nonce"); // 96-bits; unique per message

    let mut buffer: Vec<u8, U128> = Vec::new();

    let buf_result = buffer.extend_from_slice(b"plaintext message");
    //    match buf_result {
    //        Ok(()) => {
    // Encrypt `buffer` in-place, replacing the plaintext contents with ciphertext
    cipher
        .encrypt_in_place(nonce, b"", &mut buffer)
        .expect("encryption failure!");

    // `buffer` now contains the message ciphertext
    assert_ne!(&buffer, b"plaintext message");

    // Decrypt `buffer` in-place, replacing its ciphertext context with the original plaintext
    cipher
        .decrypt_in_place(nonce, b"", &mut buffer)
        .expect("decryption failure!");
    assert_eq!(&buffer, b"plaintext message");
    return "ok";
    /*        }
        Err(_e) => return "Encryption Failure",
    }*/
    return "ERROR: ENCRYPTION NOT POSSIBLE";
}
pub fn encrypt() {}
