use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
pub struct SearchResult {
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
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, Default)]
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
