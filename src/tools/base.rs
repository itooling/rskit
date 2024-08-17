use base64::prelude::*;

/// string to base64
pub fn atob(s: &str) -> String {
    BASE64_STANDARD.encode(s)
}

/// base64 to string
pub fn btoa(s: &str) -> String {
    match BASE64_STANDARD.decode(s) {
        Ok(res) => match String::from_utf8(res) {
            Ok(res) => res,
            Err(_e) => String::from(""),
        },
        Err(_e) => String::from(""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atob_test() {
        let s = "hello world";
        let res = atob(s);
        println!("to str is: {}", res);
    }

    #[test]
    fn btoa_test() {
        let s = "hello world";
        let ss = atob(s);
        let res = btoa(ss.as_str());
        println!("from str is: {}", res);
    }
}
