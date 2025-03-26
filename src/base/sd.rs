use bincode;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use percent_encoding::{NON_ALPHANUMERIC, percent_decode_str, utf8_percent_encode};
use serde::{Deserialize, Serialize};

pub const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";
pub fn sdt<S: serde::Serializer>(data: &DateTime<Local>, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&data.format(DATETIME_FORMAT).to_string())
}

pub fn ddt<'de, D: serde::Deserializer<'de>>(d: D) -> Result<DateTime<Local>, D::Error> {
    let s = String::deserialize(d)?;
    println!("s is {}", s);
    let naive =
        NaiveDateTime::parse_from_str(&s, DATETIME_FORMAT).map_err(serde::de::Error::custom)?;

    match Local.from_local_datetime(&naive).single() {
        Some(dt) => Ok(dt),
        None => Err("deserialize datetime error").map_err(serde::de::Error::custom),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Aoo {
    name: String,
    age: i32,
    #[serde(serialize_with = "sdt", deserialize_with = "ddt")]
    date: DateTime<Local>,
}

/// Serialize to binrary
pub fn to_bin<T: Serialize>(o: T) -> Result<Vec<u8>, String> {
    match bincode::serialize(&o) {
        Ok(s) => Ok(s),
        Err(e) => Err(format!("can not serialize bin: {:?}", e)),
    }
}

/// Deserialize from binrary
pub fn from_bin<'de, T: Deserialize<'de>>(b: &'de [u8]) -> Result<T, String> {
    match bincode::deserialize(b) {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("can not deserialize bin: {:?}", e)),
    }
}

/// Serialize to json string
pub fn to_json<T: Serialize>(o: T) -> Result<String, String> {
    match serde_json::to_string(&o) {
        Ok(s) => Ok(s),
        Err(e) => Err(format!("can not serialize json: {:?}", e)),
    }
}

/// Deserialize from json string
pub fn from_json<'de, T: Deserialize<'de>>(s: &'de str) -> Result<T, String> {
    match serde_json::from_str(s) {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("can not deserialize json: {:?}", e)),
    }
}

pub fn struct_field_iter<T>(t: &T) -> Option<String>
where
    T: Serialize,
{
    if let Ok(o) = serde_json::to_value(t) {
        match o {
            serde_json::Value::Null => return None,
            serde_json::Value::Bool(b) => return Some(b.to_string()),
            serde_json::Value::Number(n) => return Some(n.to_string()),
            serde_json::Value::String(s) => return Some(s),
            serde_json::Value::Array(arr) => {
                return Some(
                    arr.into_iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                );
            }
            serde_json::Value::Object(map) => {
                let mut tmp = vec![];
                for (k, v) in map {
                    let item = format!("{}={}", k, v);
                    tmp.push(item);
                }
                tmp.sort();
                return Some(tmp.join("&"));
            }
        }
    }
    None
}

pub fn url_encode(s: &str) -> String {
    utf8_percent_encode(s, NON_ALPHANUMERIC).to_string()
}

pub fn url_decode(s: &str) -> String {
    percent_decode_str(s).decode_utf8_lossy().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_json() {
        let aoo = Aoo {
            name: String::from("ok"),
            age: 18,
            date: Local::now(),
        };

        match to_json(aoo) {
            Ok(s) => {
                println!("aoo is {}", s);
            }
            Err(e) => println!("to_json error: {}", e),
        }
    }

    #[test]
    fn test_from_json() {
        let s = r#"{"name":"ok","age":18, "date": "2024-08-15 11:00:16.100"}"#;
        match from_json::<'_, Aoo>(s) {
            Ok(aoo) => {
                println!("aoo is: {:?}", aoo);
            }
            Err(e) => println!("from_json error: {}", e),
        }
    }

    #[test]
    fn test_struct_field_iter() {
        let aoo = Aoo {
            name: String::from("ok"),
            age: 18,
            date: Local::now(),
        };
        match struct_field_iter(&aoo) {
            Some(s) => {
                println!("struct is: {}", s);
            }
            None => println!("struct is none"),
        }
    }

    #[test]
    fn test_url_encode() {
        let s = "https://www.baidu.com/s?wd=ok&msg=你好";
        println!("url_encode is: {}", url_encode(s));
    }

    #[test]
    fn test_url_decode() {
        let s = "https://www.baidu.com/s?wd=ok&msg=你好";
        let s = url_encode(s);
        let s = s.as_str();
        println!("url_decode is: {}", url_decode(s));
    }
}
