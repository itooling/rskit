use std::str::FromStr;

use num_bigint::BigInt;

pub fn data_to_int(data: &[u8]) -> String {
    BigInt::from_bytes_be(num_bigint::Sign::Plus, data).to_string()
}

pub fn int_to_data(num: &String) -> Option<Vec<u8>> {
    if let Ok(bi) = BigInt::from_str(&num) {
        let (_, data) = bi.to_bytes_be();
        return Some(data);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_to_int() {
        let res = data_to_int("hello world".as_bytes());
        println!("res: {}", res);
    }

    #[test]
    fn test_int_to_data() {
        let num = data_to_int("hello world".as_bytes());
        let res = int_to_data(&num).unwrap();
        println!("res: {}", String::from_utf8(res).unwrap());
    }
}
