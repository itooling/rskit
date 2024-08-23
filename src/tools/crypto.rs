#![allow(dead_code)]

use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::{prelude::BASE64_STANDARD, Engine};
use md5;
use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey},
    rand_core::OsRng,
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};

use crate::tools::err::Error;
type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

/// encrypt
fn encrypt_aes_cbc(secret: &[u8], data: &[u8]) -> Vec<u8> {
    let key = &md5::compute(secret).0;
    let res = Aes128CbcEnc::new(key.into(), key.into()).encrypt_padded_vec_mut::<Pkcs7>(data);
    res.to_vec()
}

/// decrypt
fn decrypt_aes_cbc(secret: &[u8], data: &[u8]) -> Vec<u8> {
    let key = &md5::compute(secret).0;
    let res = Aes128CbcDec::new(key.into(), key.into())
        .decrypt_padded_vec_mut::<Pkcs7>(data)
        .unwrap();
    res.to_vec()
}

/// generate rsa key pair
pub fn generate_rsa_pair(mut bits: Option<usize>) -> (String, String) {
    if let None = bits {
        bits = Some(1024);
    }
    let mut rng = OsRng;
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
fn encrypt_rsa_byte(pub_key: &[u8], data: &[u8]) -> Result<Vec<u8>, Error> {
    match RsaPublicKey::from_public_key_der(pub_key) {
        Ok(pk) => {
            let mut rng = OsRng;
            pk.encrypt(&mut rng, Pkcs1v15Encrypt, data)
                .map_err(Error::custom)
        }
        Err(e) => Err(Error::custom(e.to_string())),
    }
}

/// decrypt rsa
fn decrypt_rsa_byte(pri_key: &[u8], data: &[u8]) -> Result<Vec<u8>, Error> {
    match RsaPrivateKey::from_pkcs8_der(pri_key) {
        Ok(pk) => pk.decrypt(Pkcs1v15Encrypt, data).map_err(Error::custom),
        Err(e) => Err(Error::custom(e.to_string())),
    }
}

/// encrypt rsa base
fn encrypt_rsa_base(pub_key: &str, data: &[u8]) -> Result<Vec<u8>, Error> {
    match BASE64_STANDARD.decode(pub_key) {
        Ok(pk) => match RsaPublicKey::from_public_key_der(&pk) {
            Ok(pk) => {
                let mut rng = OsRng;
                pk.encrypt(&mut rng, Pkcs1v15Encrypt, data)
                    .map_err(Error::custom)
            }
            Err(e) => Err(Error::custom(e.to_string())),
        },
        Err(e) => Err(Error::custom(e.to_string())),
    }
}

/// decrypt rsa base
fn decrypt_rsa_base(pri_key: &str, data: &[u8]) -> Result<Vec<u8>, Error> {
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
    fn test_encrypt_aes() {
        let key = b"xxx";
        let res = encrypt_aes_cbc(key, b"hello world");
        println!("aes encrypt is {:?}", res);
    }

    #[test]
    fn test_decrypt_aes() {
        let key = b"xxx";
        let des = encrypt_aes_cbc(key, b"hello world");
        let res = decrypt_aes_cbc(key, des.as_slice());
        println!("aes decrypt is {:?}", String::from_utf8(res).unwrap());
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
