use base64::prelude::*;

use super::err::BaseErr;

/// string to base64
pub fn atob(s: &str) -> Result<String, BaseErr> {
    Ok(BASE64_STANDARD.encode(s))
}

/// base64 to string
pub fn btoa(s: &str) -> Result<String, BaseErr> {
    match BASE64_STANDARD.decode(s) {
        Ok(res) => match String::from_utf8(res) {
            Ok(res) => Ok(res),
            Err(e) => Err(BaseErr::StrError(format!("from string error: {:?}", e))),
        },
        Err(e) => Err(BaseErr::StrError(format!("decode error: {:?}", e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atob_test() {
        let s = "hello world";
        let res = atob(s).unwrap();
        println!("to str is: {}", res);
    }

    #[test]
    fn btoa_test() {
        let s = "hello world";
        let ss = atob(s);
        let res = btoa(ss.unwrap().as_str()).unwrap();
        println!("from str is: {}", res);
    }
}
