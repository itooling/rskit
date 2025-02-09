use anyhow::{Error, Result};
use base64::{prelude::BASE64_STANDARD, Engine};
use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey},
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};

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
pub fn encrypt_rsa_byte(pub_key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    match RsaPublicKey::from_public_key_der(pub_key) {
        Ok(pk) => {
            let mut rng = rsa::rand_core::OsRng;
            let res = pk.encrypt(&mut rng, Pkcs1v15Encrypt, data)?;
            Ok(res)
        }
        Err(e) => Err(Error::msg(e.to_string())),
    }
}

/// decrypt rsa
pub fn decrypt_rsa_byte(pri_key: &[u8], data: &[u8]) -> Result<Vec<u8>, Error> {
    match RsaPrivateKey::from_pkcs8_der(pri_key) {
        Ok(pk) => {
            let res = pk.decrypt(Pkcs1v15Encrypt, data)?;
            Ok(res)
        }
        Err(e) => Err(Error::msg(e.to_string())),
    }
}

/// encrypt rsa base
pub fn encrypt_rsa_base(pub_key: &str, data: &[u8]) -> Result<Vec<u8>, Error> {
    match BASE64_STANDARD.decode(pub_key) {
        Ok(pk) => match RsaPublicKey::from_public_key_der(&pk) {
            Ok(pk) => {
                let mut rng = rsa::rand_core::OsRng;
                let res = pk.encrypt(&mut rng, Pkcs1v15Encrypt, data)?;
                Ok(res)
            }
            Err(e) => Err(Error::msg(e.to_string())),
        },
        Err(e) => Err(Error::msg(e.to_string())),
    }
}

/// decrypt rsa base
pub fn decrypt_rsa_base(pri_key: &str, data: &[u8]) -> Result<Vec<u8>, Error> {
    match BASE64_STANDARD.decode(pri_key) {
        Ok(pk) => match RsaPrivateKey::from_pkcs8_der(&pk) {
            Ok(pk) => {
                let res = pk.decrypt(Pkcs1v15Encrypt, data)?;
                Ok(res)
            }
            Err(e) => Err(Error::msg(e.to_string())),
        },
        Err(e) => Err(Error::msg(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
