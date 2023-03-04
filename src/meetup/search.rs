use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string_pretty};

#[derive(Serialize, Deserialize, Debug)]
pub struct Search {
    data: Data,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    results: Results,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
    pageInfo: PageInfo,
    count: i32,
    edges: Vec<Edge>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PageInfo {
    hasNextPage: bool,
    endCursor: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Edge {
    node: Node,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    id: String,
    result: SearchResult,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
/// Details about a meetup event
///
/// * `id`: id of the event
/// * `title`: title of the event
/// * `dateTime`: date of event
/// * `endTime`: end time of event
/// * `description`: description of event
/// * `duration`: duration of event
/// * `timezone`: timezone of event
/// * `eventType`: event type
/// * `currency`: currency of event
/// * `eventUrl`: url to event details
pub struct SearchResult {
    id: String,
    title: String,
    dateTime: String,
    endTime: String,
    description: String,
    duration: String,
    timezone: String,
    eventType: String,
    currency: String,
    eventUrl: String,
}

impl Search {
    pub fn new() -> Search {
        return Search {
            data: Data {
                results: Results {
                    pageInfo: PageInfo {
                        hasNextPage: false,
                        endCursor: "".to_string(),
                    },
                    count: 0,
                    edges: vec![],
                },
            },
        };
    }

    pub async fn fetch(&self) -> Result<Search, String> {
        let url = "https://www.meetup.com/gql";
        let body = json!({"operationName":"eventKeywordSearch","variables":{"first":20,"lat":43.7400016784668,"lon":-79.36000061035156,"topicCategoryId":null,"startDateRange":"2023-03-03T21:58:47-05:00[US/Eastern]","source":"EVENTS","query":"tech meetup","sortField":"RELEVANCE","city":"Toronto","state":"ON","country":"ca","zip":"M3B 0A3"},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"711dea20be1699a73645ed3e5cbbea50002ce3907fb3c04e414cd19dc49bcbc3"}}});
        let mut headers = HeaderMap::new();
        headers.insert("content-type", HeaderValue::from_static("application/json"));

        let client = reqwest::Client::new();
        match client
            .post(url)
            .json(&body)
            .send()
            .await
            .unwrap()
            .json::<Search>()
            .await
        {
            Ok(search) => return Ok(search),
            Err(e) => {
                return Err(format!("Error: {}", e));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch() {
        let search = Search::new();
        let search = search.fetch().await.unwrap();
        assert_eq!(search.data.results.count, 20);
    }
}
