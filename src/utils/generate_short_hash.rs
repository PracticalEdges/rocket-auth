use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::{thread_rng, RngCore};
use hex::{encode, decode};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn encrypt(data: &str, key: &str, iv_size: usize) -> String {
    let key: &[u8] = key.as_bytes();
    let mut iv: Vec<u8> = vec![0u8; iv_size];
    thread_rng().fill_bytes(&mut iv);

    let cipher: Cbc<Aes256, Pkcs7> = Aes256Cbc::new_from_slices(key, &iv).unwrap();

    let ciphertext: Vec<u8> = cipher.encrypt_vec(data.as_bytes());

    format!("{}",encode(&ciphertext))
}

pub fn decrypt(encrypted_data: &str, key: &str) -> String {
    let key: &[u8] = key.as_bytes();
    let parts: Vec<&str> = encrypted_data.split(":").collect();
    let iv: Vec<u8> = decode(parts[0]).unwrap();
    let data: Vec<u8> = decode(parts[1]).unwrap();

    let cipher: Cbc<Aes256, Pkcs7> = Aes256Cbc::new_from_slices(key, &iv).unwrap();

    let decrypted_data: Vec<u8> = cipher.decrypt_vec(&data).unwrap();

    String::from_utf8(decrypted_data).unwrap()
}