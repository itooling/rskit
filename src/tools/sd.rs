use bincode;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

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

fn serialize_datatime<S: serde::Serializer>(
    data: &DateTime<Local>,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(&data.format("%Y-%m-%d %H:%M:%S.%3f %z").to_string())
}

fn deserialize_datatime<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<DateTime<Local>, D::Error> {
    let s = String::deserialize(d)?;
    DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S.%3f %z")
        .map_err(serde::de::Error::custom)
        .map(|d| d.into())
}
#[derive(Debug, Serialize, Deserialize)]
struct Aoo {
    name: String,
    age: i32,
    #[serde(
        serialize_with = "serialize_datatime",
        deserialize_with = "deserialize_datatime"
    )]
    date: DateTime<Local>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tjson() {
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
    fn fjson() {
        let s = r#"{"name":"ok","age":18, "date": "2024-08-15 11:00:16.100 +0800"}"#;
        match from_json::<'_, Aoo>(s) {
            Ok(aoo) => {
                println!("aoo is: {:?}", aoo);
            }
            Err(e) => println!("from_json error: {}", e),
        }
    }
}