#![allow(dead_code)]

use std::sync::LazyLock;

use redis::{Client, Commands, FromRedisValue, RedisError};

pub static REDIS_CLIENT: LazyLock<Client> =
    LazyLock::new(|| Client::open("redis://127.0.0.1/").expect("connect redis error"));

pub fn value<V: FromRedisValue>(key: &str) -> Result<V, RedisError> {
    let mut conn = REDIS_CLIENT.clone();
    let v: V = conn.get(key)?;
    Ok(v)
}
