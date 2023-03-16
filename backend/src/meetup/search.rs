use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum EventType {
    physical,
    online,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum RsvpState {
    #[default]
    JOIN_OPEN,
    CLOSED,
    JOIN_APPROVAL,
    NOT_OPEN_YET,
}

impl Default for EventType {
    /// physical/in person is the default event type
    fn default() -> Self {
        EventType::physical
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Search {
    pub data: Data,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Data {
    pub results: Results,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Results {
    pub pageInfo: PageInfo,
    pub count: i32,
    pub edges: Vec<Edge>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PageInfo {
    pub hasNextPage: bool,
    pub endCursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Edge {
    pub node: Node,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Node {
    pub id: String,
    pub result: SearchResult,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
/// Details about a meetup event
///
/// * `id`: id of the event
/// * `title`: title of the event
/// * `dateTime`: date of event
/// * `endTime`: end time of event
/// * `description`: description of event
/// * `duration`: duration of event
/// * `timezone`: timezone of event
/// * `eventType`: event type. Default EventType::physical
/// * `currency`: currency of event
/// * `eventUrl`: url to event details
/// * `going`: number of people going to the even`
/// * `isAttending`: whether or not the user is attending the event
/// * `rsvpState`: state of RSVP
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub dateTime: String,
    pub endTime: String,
    pub description: String,
    pub duration: String,
    pub timezone: String,
    pub eventType: String,
    pub currency: String,
    pub eventUrl: String,
    pub going: Option<i32>,
    pub isAttending: Option<bool>,
    pub rsvpState: RsvpState,
}

mod request_body {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    use super::EventType;

    impl Default for super::request_body::Body {
        #[allow(dead_code)]
        fn default() -> super::request_body::Body {
            let utc: DateTime<Utc> = Utc::now();
            let start_date_range = utc.format("%Y-%m-%dT%H:%M:%S-05:00[US/Eastern]");
            return super::request_body::Body {
                operationName: "eventKeywordSearch".to_string(),
                variables: super::request_body::Variables {
                    after: "".to_string(),
                    first: 20,
                    lat: 43.7400016784668,
                    lon: -79.36000061035156,
                    topicCategoryId: None,
                    eventType: Some(EventType::physical),
                    startDateRange: start_date_range.to_string(),
                    source: "EVENTS".to_string(),
                    query: "".to_string(),
                    sortField: "RELEVANCE".to_string(),
                    city: "Toronto".to_string(),
                    state: "ON".to_string(),
                    country: "ca".to_string(),
                    zip: "M3B 0A3".to_string(),
                },
                extensions: super::request_body::Extensions {
                    persistedQuery: super::request_body::PersistedQuery {
                        version: 1,
                        sha256Hash:
                            "711dea20be1699a73645ed3e5cbbea50002ce3907fb3c04e414cd19dc49bcbc3"
                                .to_string(),
                    },
                },
            };
        }
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
        pub after: String,
        pub first: i32,
        pub lat: f64,
        pub lon: f64,
        pub eventType: Option<super::EventType>,
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
impl Default for Search {
    fn default() -> Search {
        return Search {
            data: Data {
                results: Results {
                    pageInfo: PageInfo {
                        hasNextPage: false,
                        endCursor: None,
                    },
                    count: 0,
                    edges: vec![],
                },
            },
        };
    }
}

impl Search {
    /// search for meetup events
    ///
    /// * `query`: the query to search for
    /// * `event_type`: the type of event to search for. Default EventType::physical
    /// * `cursor`: the cursor position
    /// * `first`: number of event nodes to fetch
    pub async fn search(
        &self,
        query: String,
        event_type: EventType,
        cursor: Option<String>,
        first: i32,
    ) -> Result<Search, String> {
        let url = "https://www.meetup.com/gql";

        let mut body = request_body::Body::default();
        body.variables.query = query;
        body.variables.eventType = Some(event_type);
        body.variables.after = cursor.unwrap_or("".to_string());
        body.variables.first = first;

        let mut headers = HeaderMap::new();
        headers.insert("content-type", HeaderValue::from_static("application/json"));

        let client = reqwest::Client::new();
        match client
            .post(url)
            .json(&body)
            .headers(headers)
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
    async fn test_search_pysical_events() {
        let search = Search::default();
        let result = search
            .search("tech meetups".to_string(), EventType::physical, None, 10)
            .await
            .unwrap();
        assert_eq!(result.data.results.count, 10);
        assert_eq!(result.data.results.edges.len(), 10);
    }

    #[tokio::test]
    async fn test_search_pagination() {
        let search = Search::default();
        let page_1 = search
            .search("tech meetups".to_string(), EventType::physical, None, 10)
            .await
            .unwrap();

        let page_2 = search
            .search(
                "tech meetups".to_string(),
                EventType::physical,
                page_1.data.results.pageInfo.endCursor.clone(),
                10,
            )
            .await
            .unwrap();

        let page_1 = serde_json::to_string(&page_1).unwrap();
        let page_2 = serde_json::to_string(&page_2).unwrap();
        // page sure page 1 and 2 are not the same
        assert_ne!(page_1, page_2);
    }
}
