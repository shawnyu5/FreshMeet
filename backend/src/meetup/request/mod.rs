pub mod common;
pub mod event_keyword_search;
pub mod get_your_events_suggested_events;
use anyhow::{anyhow, Result};
use hyper::{http::HeaderValue, HeaderMap};
use serde::{de::DeserializeOwned, Serialize};

/// Send a post request to the Meetup API
///
/// request_body: the request body to send
/// T: the type of the request body
/// R: the type of response to return
/// returns: a Result containing the response of type R
pub async fn post<T, R>(request_body: &T) -> Result<R>
where
    T: Serialize + std::fmt::Debug,
    R: DeserializeOwned + std::fmt::Debug,
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
    // return Err(anyhow!("AHHH"));

    match response.json::<R>().await {
        Ok(search) => {
            return Ok(search);
        }
        Err(e) => {
            return Err(anyhow!(e));
        }
    }
}
