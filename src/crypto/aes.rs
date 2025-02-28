use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit, block_padding::Pkcs7};
use rand::Rng;

use aes_gcm::{AeadCore, Aes128Gcm, Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;
type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

pub fn gen_rand_string(mut len: Option<usize>) -> String {
    if let None = len {
        len = Some(16);
    }
    let res: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(len.unwrap())
        .map(char::from)
        .collect();
    // println!("res is: {}", res);
    res
}

/// key 16 byte
/// encrypt cbc 128
pub fn encrypt_aes_cbc_128(key: &[u8], data: &[u8]) -> Vec<u8> {
    let iv = &key[..16];
    let res = Aes128CbcEnc::new(key.into(), iv.into()).encrypt_padded_vec_mut::<Pkcs7>(data);
    res.to_vec()
}

/// key 16 byte
/// decrypt cbc 128
pub fn decrypt_aes_cbc_128(key: &[u8], data: &[u8]) -> Vec<u8> {
    let iv = &key[..16];
    let res = Aes128CbcDec::new(key.into(), iv.into())
        .decrypt_padded_vec_mut::<Pkcs7>(data)
        .unwrap();
    res.to_vec()
}

/// key 32 byte
/// encrypt cbc 256
pub fn encrypt_aes_cbc_256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let iv = &key[..16];
    let res = Aes256CbcEnc::new(key.into(), iv.into()).encrypt_padded_vec_mut::<Pkcs7>(data);
    res.to_vec()
}

/// key 32 byte
/// decrypt cbc 256
pub fn decrypt_aes_cbc_256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let iv = &key[..16];
    let res = Aes256CbcDec::new(key.into(), iv.into())
        .decrypt_padded_vec_mut::<Pkcs7>(data)
        .unwrap();
    res.to_vec()
}

pub fn aes_gcm_key_128() -> Vec<u8> {
    Aes128Gcm::generate_key(aes_gcm::aead::OsRng).to_vec()
}

pub fn aes_gcm_nonce_128() -> Vec<u8> {
    Aes128Gcm::generate_nonce(aes_gcm::aead::OsRng).to_vec()
}

pub fn aes_gcm_key_256() -> Vec<u8> {
    Aes256Gcm::generate_key(aes_gcm::aead::OsRng).to_vec()
}

pub fn aes_gcm_nonce_256() -> Vec<u8> {
    Aes256Gcm::generate_nonce(aes_gcm::aead::OsRng).to_vec()
}

/// encrypt gcm 128
pub fn encrypt_aes_gcm_128(secret: &[u8], nonce: &[u8], data: &[u8]) -> Vec<u8> {
    let key = Key::<Aes128Gcm>::from_slice(secret);
    let nonce = Nonce::from_slice(nonce);
    let cipher = Aes128Gcm::new(&key);
    cipher.encrypt(&nonce, data).unwrap()
}

/// decrypt gcm 128
pub fn decrypt_aes_gcm_128(secret: &[u8], nonce: &[u8], data: &[u8]) -> Vec<u8> {
    let key = Key::<Aes128Gcm>::from_slice(secret);
    let nonce = Nonce::from_slice(nonce);
    let cipher = Aes128Gcm::new(&key);
    cipher.decrypt(&nonce, data).unwrap()
}

/// encrypt gcm 256
pub fn encrypt_aes_gcm_256(secret: &[u8], nonce: &[u8], data: &[u8]) -> Vec<u8> {
    let key = Key::<Aes256Gcm>::from_slice(secret);
    let nonce = Nonce::from_slice(nonce);
    let cipher = Aes256Gcm::new(&key);
    cipher.encrypt(&nonce, data).unwrap()
}

/// decrypt gcm 256
pub fn decrypt_aes_gcm_256(secret: &[u8], nonce: &[u8], data: &[u8]) -> Vec<u8> {
    let key = Key::<Aes256Gcm>::from_slice(secret);
    let nonce = Nonce::from_slice(nonce);
    let cipher = Aes256Gcm::new(&key);
    cipher.decrypt(&nonce, data).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_rand_string() {
        gen_rand_string(None);
    }

    #[test]
    fn test_aes_cbc_128() {
        let key = gen_rand_string(Some(16));
        let key = key.as_bytes();
        let des = encrypt_aes_cbc_128(key, b"hello world");
        let res = decrypt_aes_cbc_128(key, des.as_slice());
        println!(
            "aes cbc 128 decrypt is {:?}",
            String::from_utf8(res).unwrap()
        );
    }

    #[test]
    fn test_aes_cbc_256() {
        let key = gen_rand_string(Some(32));
        let key = key.as_bytes();
        let des = encrypt_aes_cbc_256(key, b"hello world");
        let res = decrypt_aes_cbc_256(key, des.as_slice());
        println!(
            "aes cbc 256 decrypt is {:?}",
            String::from_utf8(res).unwrap()
        );
    }

    #[test]
    fn test_aes_gcm_128() {
        let key = aes_gcm_key_128();
        let nonce = aes_gcm_nonce_128();
        let des = encrypt_aes_gcm_128(&key, &nonce, b"hello world");
        let res = decrypt_aes_gcm_128(&key, &nonce, des.as_slice());
        println!(
            "aes gcm 128 decrypt is {:?}",
            String::from_utf8(res).unwrap()
        );
    }

    #[test]
    fn test_aes_gcm_256() {
        let key = aes_gcm_key_256();
        let nonce = aes_gcm_nonce_256();
        let des = encrypt_aes_gcm_256(&key, &nonce, b"hello world");
        let res = decrypt_aes_gcm_256(&key, &nonce, des.as_slice());
        println!(
            "aes gcm 256 decrypt is {:?}",
            String::from_utf8(res).unwrap()
        );
    }
}
