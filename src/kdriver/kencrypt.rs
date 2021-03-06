use aes_gcm_siv::aead::heapless::{consts::U128, Vec};

use aes_gcm_siv::aead::{
    generic_array::{ArrayLength, GenericArray},
    AeadInPlace, NewAead,
};

use aes_gcm_siv::AesGcmSiv; // Or `Aes128GcmSiv`

pub fn test_encrypt() -> &'static str {
    /* Not implemented because I'm stupid
        let key: GenericArray<u8, ArrayLength<T>> =
            GenericArray::from_slice(b"an example very very secret key.");
        let cipher = AesGcmSiv::new(&key);

        let nonce = GenericArray::from_slice(b"unique nonce"); // 96-bits; unique per message

        let mut buffer: Vec<u8, U128> = Vec::new();
        buffer.extend_from_slice(b"plaintext message");

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
    */
    return "ERROR: ENCRYPTION NOT POSSIBLE";
}
pub fn encrypt() {}
