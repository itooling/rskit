#![allow(dead_code)]

use std::sync::LazyLock;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, sqlite::SqliteConnectOptions, SqlitePool};

pub static SPOOL: LazyLock<SqlitePool> = LazyLock::new(|| {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            let pool = SqlitePool::connect_with(
                SqliteConnectOptions::new()
                    .create_if_missing(true)
                    .filename("kit.db"),
            )
            .await
            .expect("connect sql error");

            let sql = r#"
                create table if not exists user (
                    id integer primary key autoincrement,
                    created_at timestamp default current_timestamp,
                    updated_at timestamp default current_timestamp,
                    created_by text default 'sys',
                    updated_by text default 'sys',
                    username text,
                    nickname text,
                    password text,
                    age int
                )
            "#;

            sqlx::query(sql)
                .execute(&pool)
                .await
                .expect("create table error");

            pool
        })
    })
});

#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
struct User {
    id: Option<u64>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    created_by: Option<String>,
    updated_by: Option<String>,
    username: Option<String>,
    nickname: Option<String>,
    password: Option<String>,
    age: Option<u32>,
}

impl User {
    fn new() -> User {
        User {
            username: Some("zhangsan".to_string()),
            nickname: Some("张三".to_string()),
            password: Some("123456".to_string()),
            age: Some(18),
            ..Default::default()
        }
    }
    async fn list() -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("select * from user")
            .fetch_all(&*SPOOL)
            .await
    }

    async fn insert(&self) -> Result<(), sqlx::Error> {
        let sql = r#"
            insert into user (username, nickname, password, age) 
            values (?,?,?,?)
            "#;

        sqlx::query(sql)
            .bind(&self.username)
            .bind(&self.nickname)
            .bind(&self.password)
            .bind(&self.age)
            .fetch_one(&*SPOOL)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_insert() {
        let _ = tokio::runtime::Handle::current()
            .spawn_blocking(|| {
                let _ = &*SPOOL;
            })
            .await;

        let user = User::new();
        match user.insert().await {
            Ok(_) => {
                println!("insert success");
            }
            Err(e) => println!("select user error: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_list() {
        let _ = tokio::runtime::Handle::current()
            .spawn_blocking(|| {
                let _ = &*SPOOL;
            })
            .await;

        match User::list().await {
            Ok(users) => {
                println!("users: {:?}", serde_json::to_string(&users).unwrap());
            }
            Err(e) => println!("select user error: {:?}", e),
        }
    }
}
