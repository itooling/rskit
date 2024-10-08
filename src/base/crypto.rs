#![allow(dead_code)]

use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::{prelude::BASE64_STANDARD, Engine};
use rand::Rng;
use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey},
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};

use aes_gcm::{aead::Aead, AeadCore, Aes128Gcm, Aes256Gcm, Key, KeyInit, Nonce};

use crate::base::err::Error;

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

/// generate rsa key pair
pub fn generate_rsa_pair(mut bits: Option<usize>) -> (String, String) {
    if let None = bits {
        bits = Some(1024);
    }
    let mut rng = rsa::rand_core::OsRng;
    let pri_key = RsaPrivateKey::new(&mut rng, bits.unwrap()).unwrap();
    let pub_key = RsaPublicKey::from(&pri_key);
    (
        BASE64_STANDARD.encode(EncodePrivateKey::to_pkcs8_der(&pri_key).unwrap().as_bytes()),
        BASE64_STANDARD.encode(
            EncodePublicKey::to_public_key_der(&pub_key)
                .unwrap()
                .as_bytes(),
        ),
    )
}

/// encrypt rsa
pub fn encrypt_rsa_byte(pub_key: &[u8], data: &[u8]) -> Result<Vec<u8>, Error> {
    match RsaPublicKey::from_public_key_der(pub_key) {
        Ok(pk) => {
            let mut rng = rsa::rand_core::OsRng;
            pk.encrypt(&mut rng, Pkcs1v15Encrypt, data)
                .map_err(Error::custom)
        }
        Err(e) => Err(Error::custom(e.to_string())),
    }
}

/// decrypt rsa
pub fn decrypt_rsa_byte(pri_key: &[u8], data: &[u8]) -> Result<Vec<u8>, Error> {
    match RsaPrivateKey::from_pkcs8_der(pri_key) {
        Ok(pk) => pk.decrypt(Pkcs1v15Encrypt, data).map_err(Error::custom),
        Err(e) => Err(Error::custom(e.to_string())),
    }
}

/// encrypt rsa base
pub fn encrypt_rsa_base(pub_key: &str, data: &[u8]) -> Result<Vec<u8>, Error> {
    match BASE64_STANDARD.decode(pub_key) {
        Ok(pk) => match RsaPublicKey::from_public_key_der(&pk) {
            Ok(pk) => {
                let mut rng = rsa::rand_core::OsRng;
                pk.encrypt(&mut rng, Pkcs1v15Encrypt, data)
                    .map_err(Error::custom)
            }
            Err(e) => Err(Error::custom(e.to_string())),
        },
        Err(e) => Err(Error::custom(e.to_string())),
    }
}

/// decrypt rsa base
pub fn decrypt_rsa_base(pri_key: &str, data: &[u8]) -> Result<Vec<u8>, Error> {
    match BASE64_STANDARD.decode(pri_key) {
        Ok(pk) => match RsaPrivateKey::from_pkcs8_der(&pk) {
            Ok(pk) => pk.decrypt(Pkcs1v15Encrypt, data).map_err(Error::custom),
            Err(e) => Err(Error::custom(e.to_string())),
        },
        Err(e) => Err(Error::custom(e.to_string())),
    }
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

    #[test]
    fn test_generate_rsa_pair() {
        let res = generate_rsa_pair(None);
        println!("key pair: {:?}", res);
    }

    #[test]
    fn test_rsa() {
        let (pri_key, pub_key) = generate_rsa_pair(None);
        let src = "hello world";
        let enc = encrypt_rsa_base(&pub_key, src.as_bytes()).unwrap();
        let res = decrypt_rsa_base(&pri_key, &enc).unwrap();
        println!("src: {}", src);
        println!("res: {}", String::from_utf8(res).unwrap());
    }
}
