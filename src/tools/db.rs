#![allow(dead_code)]

use std::sync::LazyLock;

use sqlx::{prelude::FromRow, sqlite::SqliteConnectOptions, SqlitePool};

pub static SPOOL: LazyLock<SqlitePool> = LazyLock::new(|| {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            SqlitePool::connect_with(SqliteConnectOptions::new().filename("test.db"))
                .await
                .expect("connect sql error")
        })
    })
});

#[derive(Debug, FromRow)]
struct User {
    id: u64,
    name: Option<String>,
    age: Option<u32>,
}

async fn get_user() -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("select * from user_info")
        .fetch_all(&*SPOOL)
        .await
}
