use sha2::{Digest, Sha256, Sha512};

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
    fn sh256() {
        let res = hash_256("hello world");
        println!("hash 256 is {}", res);
    }

    #[test]
    fn sh512() {
        let res = hash_512("hello world");
        println!("hash 512 is {}", res);
    }
}
