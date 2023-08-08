use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResponse {
    pub data: Data,
}

impl Default for SearchResponse {
    fn default() -> SearchResponse {
        return SearchResponse {
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

impl SearchResponse {
    /// return all events from the search result
    pub fn events(&self) -> Vec<Event> {
        self.data
            .results
            .edges
            .iter()
            .map(|e| e.node.result.clone())
            .collect()
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
    pub result: Event,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone, Ord, Eq, PartialOrd)]
/// a single meetup event
pub struct Event {
    pub id: String,
    pub title: String,
    pub dateTime: String,
    pub endTime: String,
    pub description: String,
    pub venue: Option<Venue>,
    pub duration: String,
    pub timezone: String,
    pub eventType: String,
    pub currency: String,
    pub eventUrl: String,
    pub going: Option<i32>,
    pub rsvpState: RsvpState,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone, Ord, Eq, PartialOrd)]
pub struct Venue {
    pub id: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub name: String,
    pub radius: i64,
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
