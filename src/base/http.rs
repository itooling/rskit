use std::sync::LazyLock;

use reqwest::{Client, IntoUrl, Method, Response};
use serde::Serialize;

pub static HTTP_CLIENT: LazyLock<Client> =
    LazyLock::new(|| Client::builder().build().expect("create http client error"));

pub async fn request<U, D>(method: Method, url: U, data: &D) -> Result<Response, ()>
where
    U: IntoUrl,
    D: Serialize + ?Sized,
{
    match HTTP_CLIENT.request(method, url).json(data).send().await {
        Ok(res) => Ok(res),
        Err(e) => {
            log::error!("request error: {:?}", e);
            Err(())
        }
    }
}
