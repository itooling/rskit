use base64::{prelude::BASE64_STANDARD, Engine};
use sha2::{Digest, Sha256, Sha512};

use crate::tools::err::Error;

/// encode
pub fn atob(s: &[u8]) -> Result<String, Error> {
    Ok(BASE64_STANDARD.encode(s))
}

/// decode
pub fn btoa(s: &str) -> Result<Vec<u8>, Error> {
    BASE64_STANDARD.decode(s).map_err(Error::custom)
}

pub fn sha_256(s: &[u8]) -> String {
    let mut sh = Sha256::new();
    sh.update(s);
    let res = sh.finalize();
    return format!("{:x}", res);
}

pub fn sha_512(s: &[u8]) -> String {
    let mut sh = Sha512::new();
    sh.update(s);
    let res = sh.finalize();
    return format!("{:x}", res);
}

pub fn hash_256(s: &str) -> String {
    let mut sh = Sha256::new();
    sh.update(s);
    let res = sh.finalize();
    return format!("{:x}", res);
}

pub fn hash_512(s: &str) -> String {
    let mut sh = Sha512::new();
    sh.update(s);
    let res = sh.finalize();
    return format!("{:x}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atob() {
        let s = "hello world";
        let res = atob(s.as_bytes()).unwrap();
        println!("res is: {}", res);
    }

    #[test]
    fn test_btoa() {
        let s = "hello world";
        let ss = atob(s.as_bytes()).unwrap();
        let res = btoa(&ss).unwrap();
        println!("res is: {}", String::from_utf8(res).unwrap());
    }

    #[test]
    fn test_sh256() {
        let res = hash_256("hello world");
        println!("hash 256 is {}", res);
    }

    #[test]
    fn test_sh512() {
        let res = hash_512("hello world");
        println!("hash 512 is {}", res);
    }
}
