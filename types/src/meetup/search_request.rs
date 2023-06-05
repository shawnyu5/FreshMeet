use std::fmt::Display;

use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

/// types of events a meetup can be
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EventType {
    physical,
    online,
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::physical => write!(f, "PHYSICAL"),
            EventType::online => write!(f, "ONLINE"),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone, Ord, Eq, PartialOrd)]
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
pub struct SearchResult {
    pub data: Data,
}

impl SearchResult {
    /// return all events from the search result
    pub fn events(&self) -> Vec<Result_> {
        self.data
            .results
            .edges
            .iter()
            .map(|e| e.node.result.clone())
            .collect()
    }
}

impl Default for SearchResult {
    fn default() -> SearchResult {
        return SearchResult {
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
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
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
    pub result: Result_,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone, Ord, Eq, PartialOrd)]
/// Details about a meetup event
pub struct Result_ {
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
    pub isAttending: bool,
    pub rsvpState: RsvpState,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Variables {
    pub after: String,
    pub first: i32,
    pub lat: f64,
    pub lon: f64,
    pub eventType: Option<EventType>,
    pub topicCategoryId: Option<String>,
    pub startDateRange: String,
    pub startDate: Option<String>,
    pub source: String,
    pub query: String,
    pub sortField: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub zip: String,
}

impl Default for Variables {
    fn default() -> Self {
        let utc: DateTime<Utc> = Utc::now();
        let start_date_range = utc.format("%Y-%m-%dT%H:%M:%S-05:00[US/Eastern]");
        let today = Local::now().format("%Y-%m-%dT%H:%M:%S-05:00[US/Eastern]");
        Self {
            after: "".to_string(),
            first: 20,
            lat: 43.7400016784668,
            lon: -79.36000061035156,
            topicCategoryId: None,
            eventType: Some(EventType::physical),
            startDateRange: start_date_range.to_string(),
            startDate: Some(today.to_string()),
            source: "EVENTS".to_string(),
            query: "".to_string(),
            sortField: "RELEVANCE".to_string(),
            city: "Toronto".to_string(),
            state: "ON".to_string(),
            country: "ca".to_string(),
            zip: "M3B 0A3".to_string(),
        }
    }
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
