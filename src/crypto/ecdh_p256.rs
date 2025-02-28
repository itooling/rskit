use anyhow::{Error, Result};
use p256::{PublicKey, SecretKey, ecdh::diffie_hellman};
use rand::thread_rng;

pub fn generate_keypair() -> (String, String) {
    let sk = SecretKey::random(&mut thread_rng());
    let sk_bytes = sk.to_bytes().to_vec();

    let pk = sk.public_key();
    let pk_bytes = pk.to_sec1_bytes();

    let sk_str = hex::encode(sk_bytes);
    let pk_str = hex::encode(pk_bytes);

    //println!("sk:{}", &sk_str);
    //println!("pk:{}", &pk_str);

    (sk_str, pk_str)
}

pub fn generate_shared(sk: String, pk: String) -> Result<String, Error> {
    if let Ok(sks) = hex::decode(sk) {
        if let Ok(pks) = hex::decode(pk) {
            let secret_key = SecretKey::from_slice(&sks).expect("secret key error");
            let public_key = PublicKey::from_sec1_bytes(&pks).expect("publick key error");
            let shared_secret =
                diffie_hellman(secret_key.to_nonzero_scalar(), public_key.as_affine());
            let res = hex::encode(shared_secret.raw_secret_bytes());
            return Ok(res);
        }
    }
    Err(Error::msg("generate shared error"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        generate_keypair();
    }

    #[test]
    fn test_generate_shared() {
        println!("test_generate_shared");
        let p1 = generate_keypair();
        let p2 = generate_keypair();
        let res1 = generate_shared(p1.0, p2.1).unwrap();
        let res2 = generate_shared(p2.0, p1.1).unwrap();
        println!("shared key1 is:{}", res1);
        println!("shared key2 is:{}", res2);
    }
}
