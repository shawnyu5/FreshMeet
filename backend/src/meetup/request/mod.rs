pub mod event_keyword_search;
pub mod request;
use anyhow::{anyhow, Result};
use hyper::{http::HeaderValue, HeaderMap};
use serde::{de::DeserializeOwned, Serialize};

/// Make a request to the meetup api
///
/// request_body: the request body to send
/// R: the type of response to return, wrapped in a Result
pub async fn search<T, R>(request_body: &T) -> Result<R>
where
    T: Serialize,
    R: DeserializeOwned,
{
    let url = "https://www.meetup.com/gql";
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .json(&request_body)
        .headers(headers)
        .send()
        .await
        .unwrap();

    // dbg!(&response.text().await?);
    // return Ok(SearchResponse::default());

    match response.json::<R>().await {
        Ok(search) => {
            return Ok(search);
        }
        Err(e) => {
            return Err(anyhow!(e));
        }
    }
}
