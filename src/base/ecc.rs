use ed25519_dalek::{Signature, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use rsa::signature::SignerMut;

pub fn generate_sec() -> (SigningKey, VerifyingKey) {
    let mut rng = OsRng;
    let sk = SigningKey::generate(&mut rng);
    let vk = sk.verifying_key();
    (sk, vk)
}

pub fn signature(sk: &mut SigningKey, msg: &[u8]) -> Signature {
    sk.sign(msg)
}

pub fn verify(vk: &VerifyingKey, s: &Signature, msg: &[u8]) -> bool {
    match vk.verify_strict(msg, s) {
        Ok(_) => true,
        Err(_) => false,
    }
}
