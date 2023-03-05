use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::json;

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

mod request_body {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    #[allow(dead_code)]
    /// create a new request body for the meetup search
    ///
    /// * `num_of_events`: number of events to return. Default 20
    /// * `query`: the query for the search
    pub fn new(num_of_events: Option<i32>, query: String) -> super::request_body::Body {
        let utc: DateTime<Utc> = Utc::now();
        let num_of_events = num_of_events.unwrap_or(20);
        let start_date_range = utc.format("%Y-%m-%dT%H:%M:%S-05:00[US/Eastern]");
        // TODO: read up on API docs on the spec if this API
        // https://www.meetup.com/api/schema/#event:~:text=something%20gone%20wrong-,Event,-object
        return super::request_body::Body {
            operationName: "eventKeywordSearch".to_string(),
            variables: super::request_body::Variables {
                first: num_of_events,
                lat: 43.7400016784668,
                lon: -79.36000061035156,
                topicCategoryId: None,
                startDateRange: start_date_range.to_string(),
                source: "EVENTS".to_string(),
                query,
                sortField: "RELEVANCE".to_string(),
                city: "Toronto".to_string(),
                state: "ON".to_string(),
                country: "ca".to_string(),
                zip: "M3B 0A3".to_string(),
            },
            extensions: super::request_body::Extensions {
                persistedQuery: super::request_body::PersistedQuery {
                    version: 1,
                    sha256Hash: "711dea20be1699a73645ed3e5cbbea50002ce3907fb3c04e414cd19dc49bcbc3"
                        .to_string(),
                },
            },
        };
    }
    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Body {
        pub operationName: String,
        pub variables: Variables,
        pub extensions: Extensions,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Variables {
        pub first: i32,
        pub lat: f64,
        pub lon: f64,
        pub topicCategoryId: Option<String>,
        pub startDateRange: String,
        pub source: String,
        pub query: String,
        pub sortField: String,
        pub city: String,
        pub state: String,
        pub country: String,
        pub zip: String,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Extensions {
        pub persistedQuery: PersistedQuery,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct PersistedQuery {
        version: i32,
        sha256Hash: String,
    }
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

    /// search for meetup events
    ///
    /// * `query`: the query to search for. Default: tech meetups
    pub async fn fetch(&self, query: Option<String>) -> Result<Search, String> {
        let url = "https://www.meetup.com/gql";
        let query = query.unwrap_or("tech meetups".to_string());
        let body = request_body::new(Some(20), query);

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
                return Err(format!("error: {}", e));
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
        let search = search
            .fetch(Some("tech meetups".to_string()))
            .await
            .unwrap();
        assert_eq!(search.data.results.count, 20);
    }
}
