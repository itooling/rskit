use anyhow::Result;
use base58::{FromBase58, ToBase58};
use base64::{prelude::BASE64_STANDARD, Engine};
use sha2::{Digest, Sha256, Sha512};

pub fn base64_encode(s: &[u8]) -> Result<String> {
    Ok(BASE64_STANDARD.encode(s))
}

pub fn base64_decode(s: &str) -> Result<Vec<u8>> {
    let res = BASE64_STANDARD.decode(s)?;
    Ok(res)
}

pub fn base58_encode(s: &[u8]) -> Result<String> {
    Ok(s.to_base58())
}

pub fn base58_decode(s: &str) -> Result<Vec<u8>> {
    let res = s
        .from_base58()
        .map_err(|e| anyhow::anyhow!(format!("{e:?}")))?;
    Ok(res)
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

pub fn pwstr_to_string(pwstr: *const u16) -> Option<String> {
    if !pwstr.is_null() {
        let len = unsafe {
            let mut len = 0;
            while *pwstr.offset(len) != 0 {
                len += 1;
            }
            len
        };
        let data = unsafe { std::slice::from_raw_parts(pwstr, len as usize) };
        return Some(String::from_utf16_lossy(data));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        let s = "hello world";
        let res = base64_encode(s.as_bytes()).unwrap();
        println!("res is: {}", res);
    }

    #[test]
    fn test_base64_decode() {
        let s = "hello world";
        let ss = base64_encode(s.as_bytes()).unwrap();
        let res = base64_decode(&ss).unwrap();
        println!("res is: {}", String::from_utf8(res).unwrap());
    }

    #[test]
    fn test_base58_encode() {
        let s = "hello world";
        let res = base58_encode(s.as_bytes()).unwrap();
        println!("res is: {}", res);
    }

    #[test]
    fn test_base58_decode() {
        let s = "hello world";
        let ss = base58_encode(s.as_bytes()).unwrap();
        let res = base58_decode(&ss).unwrap();
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
