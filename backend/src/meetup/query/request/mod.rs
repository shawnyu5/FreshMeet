use anyhow::{anyhow, Result};
use axum::http::HeaderValue;
use chrono::Utc;
use hyper::HeaderMap;
use serde::{de::DeserializeOwned, Serialize};
use tracing::{debug, error};

pub mod gql2;

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
    let url = "https://www.meetup.com/gql2";
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("application/json"));
    let timestamp = Utc::now().timestamp();
    headers.insert(
        "cookie",
        HeaderValue::from_static("MEETUP_MEMBER=id=284126435&status=1&timestamp=1722905619&bs=0&ql=false&s=fbf4f42e2d618fd7fdc9054dadd7514a82024b03&scope=ALL; MEETUP_LANGUAGE=language=en&country=US; memberId=284126435;"),
    );

    let client = reqwest::Client::new();
    debug!("Making request");
    debug!(
        "Request body json: {}",
        serde_json::to_string(request_body).unwrap()
    );
    let response = match client
        .post(url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await
    {
        Ok(res) => {
            debug!("Successfully make request");
            res
        }
        Err(e) => {
            error!("Status: {}", &e.status().unwrap_or_default());
            return Err(anyhow!("Failed to make request: {}", e));
        }
    };

    // dbg!(&response.text().await?);
    // return Err(anyhow!("AHHH"));

    match response.json::<R>().await {
        Ok(json) => {
            return Ok(json);
        }
        Err(e) => {
            return Err(anyhow!("Failed to parse JSON: {}", e));
        }
    }
}
