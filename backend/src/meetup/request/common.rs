/// common types for request bodies to Meetup api
use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
            EventType::physical => write!(f, "physical"),
            EventType::online => write!(f, "online"),
        }
    }
}

impl Default for EventType {
    /// physical/in person is the default event type
    fn default() -> Self {
        EventType::physical
    }
}

/// Types of oepration names for requests to the Meetup api
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum OperationName {
    /// search for events by keywords
    eventKeywordSearch,
    /// get suggested events
    getYourEventsSuggestedEvents,
}

impl Display for OperationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationName::eventKeywordSearch => write!(f, "eventKeywordSearch"),
            OperationName::getYourEventsSuggestedEvents => {
                write!(f, "getYourEventsSuggestedEvents")
            }
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    persisted_query: PersistedQuery,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistedQuery {
    sha256_hash: String,
    version: i32,
}

impl Default for PersistedQuery {
    fn default() -> Self {
        Self {
            sha256_hash: "4e6f93789cdacfe8809c93b09549c7e5cd019375e653a8ee439a963b1fd91c5e"
                .to_string(),
            version: 1,
        }
    }
}
