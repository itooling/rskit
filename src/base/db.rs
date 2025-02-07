#![allow(dead_code)]

use std::{env, sync::LazyLock};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::PgPoolOptions,
    prelude::{FromRow, Type},
    sqlite::SqliteConnectOptions,
    PgPool, SqlitePool,
};

pub static LITE_POOL: LazyLock<SqlitePool> = LazyLock::new(|| {
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
                    age integer,
                    sex text
                )
            "#;

            sqlx::query(sql)
                .execute(&pool)
                .await
                .expect("create table user error");

            let sql = r#"
                create table if not exists work (
                    id integer primary key autoincrement,
                    post text
                )
            "#;

            sqlx::query(sql)
                .execute(&pool)
                .await
                .expect("create table work error");

            pool
        })
    })
});

pub static PG_POOL: LazyLock<PgPool> = LazyLock::new(|| {
    tokio::runtime::Handle::current().block_on(async {
        dotenvy::dotenv().expect("init env error");
        let database = env::var("DATABASE_URL")
            .unwrap_or("postgres://postgres:123456@127.0.0.1:5432/one".to_string());

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database)
            .await
            .expect("connect sql error");
        pool
    })
});

#[derive(Debug, Default, Serialize, Deserialize, Type)]
enum Sex {
    Male,
    Female,
    #[default]
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
struct User {
    id: Option<i64>,
    created_at: Option<DateTime<Local>>,
    updated_at: Option<DateTime<Local>>,
    created_by: Option<String>,
    updated_by: Option<String>,
    username: Option<String>,
    nickname: Option<String>,
    password: Option<String>,
    age: Option<u32>,
    sex: Option<Sex>,
}

impl User {
    fn new() -> User {
        User {
            username: Some("zhangsan".to_string()),
            nickname: Some("张三".to_string()),
            password: Some("123456".to_string()),
            age: Some(18),
            sex: Some(Sex::Male),
            ..Default::default()
        }
    }

    async fn add(&self) -> Result<(), sqlx::Error> {
        let sql = r#"
            insert into user
            (username, nickname, password, age, sex) 
            values
            ($1, $2, $3, $4, $5)
            "#;

        sqlx::query(sql)
            .bind(&self.username)
            .bind(&self.nickname)
            .bind(&self.password)
            .bind(&self.age)
            .bind(&self.sex)
            .execute(&*LITE_POOL)
            .await?;

        Ok(())
    }

    async fn list() -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("select * from user")
            .fetch_all(&*LITE_POOL)
            .await
    }

    async fn get(id: i64) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("select * from user where id = ?")
            .bind(id)
            .fetch_one(&*LITE_POOL)
            .await
    }

    async fn ids(ids: &[i64]) -> Result<Vec<User>, sqlx::Error> {
        let ids = ids
            .iter()
            .map(i64::to_string)
            .collect::<Vec<String>>()
            .join(",");
        let sql = format!("select * from user where id in ({ids})");
        let sql = sql.as_str();

        sqlx::query_as::<_, User>(sql)
            .bind(ids)
            .fetch_all(&*LITE_POOL)
            .await
    }
}

#[derive(Debug, Default, FromRow)]
struct Work {
    id: Option<i64>,
    post: Option<String>,
}

impl Work {
    fn new() -> Self {
        Work {
            post: Some("it".to_string()),
            ..Default::default()
        }
    }

    async fn add(&self) -> Result<(), sqlx::Error> {
        let sql = r#"
            insert into work
            (post) 
            values
            ($1)
            "#;

        sqlx::query(sql).bind(&self.post).execute(&*PG_POOL).await?;

        Ok(())
    }

    async fn list() -> Result<Vec<Work>, sqlx::Error> {
        //sqlx::query_as!(Work, "select * from work")
        sqlx::query_as::<_, Work>("select * from work")
            .fetch_all(&*PG_POOL)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add() {
        let _ = tokio::runtime::Handle::current()
            .spawn_blocking(|| {
                let _ = &*LITE_POOL;
            })
            .await;

        let user = User::new();
        match user.add().await {
            Ok(_) => {
                println!("add success");
            }
            Err(e) => println!("add error: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_list() {
        let _ = tokio::runtime::Handle::current()
            .spawn_blocking(|| {
                let _ = &*LITE_POOL;
            })
            .await;

        match User::list().await {
            Ok(users) => {
                println!("users: {:?}", users);
            }
            Err(e) => println!("list error: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_get() {
        let _ = tokio::runtime::Handle::current()
            .spawn_blocking(|| {
                let _ = &*LITE_POOL;
            })
            .await;

        match User::get(1).await {
            Ok(user) => {
                println!("user: {:?}", user);
            }
            Err(e) => println!("get error: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_ids() {
        let _ = tokio::runtime::Handle::current()
            .spawn_blocking(|| {
                let _ = &*LITE_POOL;
            })
            .await;

        match User::ids(&[1, 2, 3]).await {
            Ok(users) => {
                println!("user: {:?}", users);
            }
            Err(e) => println!("ids error: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_add_work() {
        let _ = tokio::runtime::Handle::current()
            .spawn_blocking(|| {
                let _ = &*PG_POOL;
            })
            .await;

        let work = Work::new();
        match work.add().await {
            Ok(_) => {
                println!("add success");
            }
            Err(e) => println!("add error: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_list_work() {
        let _ = tokio::runtime::Handle::current()
            .spawn_blocking(|| {
                let _ = &*PG_POOL;
            })
            .await;

        match Work::list().await {
            Ok(works) => {
                println!("works: {:?}", works);
            }
            Err(e) => println!("list error: {:?}", e),
        }
    }
}
