use std::sync::LazyLock;

use redis::{Client, Commands, FromRedisValue, RedisError, ToRedisArgs};

pub static REDIS_CLIENT: LazyLock<Client> =
    LazyLock::new(|| Client::open("redis://127.0.0.1").expect("connect redis error"));

pub fn get<V: FromRedisValue>(key: &str) -> Result<V, RedisError> {
    let mut conn = REDIS_CLIENT.clone();
    let v: V = conn.get(key)?;
    Ok(v)
}

pub fn set<V: ToRedisArgs>(key: &str, val: V) -> Result<(), RedisError> {
    let mut conn = REDIS_CLIENT.clone();
    conn.set::<_, V, ()>(key, val)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redis() {
        set::<&str>("test", "test").unwrap();
        let v = get::<String>("test").unwrap();
        assert_eq!("test", v);
    }
}
