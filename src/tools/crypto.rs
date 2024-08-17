#![allow(dead_code)]

use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use md5;
type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

/// encrypt aes data
fn encrypt_aes_cbc(data: &[u8], secret: &[u8]) -> Vec<u8> {
    let key = &md5::compute(secret).0;
    let res = Aes128CbcEnc::new(key.into(), key.into()).encrypt_padded_vec_mut::<Pkcs7>(data);
    res.to_vec()
}

/// decrypt aes data
fn decrypt_aes_cbc(data: &[u8], secret: &[u8]) -> Vec<u8> {
    let key = &md5::compute(secret).0;
    let res = Aes128CbcDec::new(key.into(), key.into())
        .decrypt_padded_vec_mut::<Pkcs7>(data)
        .unwrap();
    res.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_aes_test() {
        let key = b"xxx";
        let res = encrypt_aes_cbc(b"hello world", key);
        println!("aes encrypt is {:?}", res);
    }

    #[test]
    fn decrypt_aes_test() {
        let key = b"xxx";
        let des = encrypt_aes_cbc(b"hello world", key);
        let res = decrypt_aes_cbc(des.as_slice(), key);
        println!("aes decrypt is {:?}", String::from_utf8(res).unwrap());
    }
}
