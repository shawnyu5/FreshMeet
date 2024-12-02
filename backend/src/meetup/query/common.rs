//! Common types for request bodies to Meetup api
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// types of events a meetup can be
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub enum EventType {
    #[default]
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

/// Types of oepration names for requests to the Meetup api
#[deprecated(note = "Use OperationName2 for the more up to date event names")]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum OperationName {
    /// search for events by keywords
    eventKeywordSearch,
    /// get suggested events
    getYourEventsSuggestedEvents,
    /// search by category
    categorySearch,
    /// Search for events with a query
    eventSearchWithSeries,
}

impl Display for OperationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationName::eventKeywordSearch => write!(f, "eventKeywordSearch"),
            OperationName::getYourEventsSuggestedEvents => {
                write!(f, "getYourEventsSuggestedEvents")
            }
            OperationName::categorySearch => {
                write!(f, "categorySearch")
            }
            OperationName::eventSearchWithSeries => write!(f, "eventSearchWithSeries"),
        }
    }
}

/// Types of oepration names for requests to the Meetup api gql2
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum OperationName2 {
    /// Get recommended events
    recommendedEventsWithSeries,
    /// Search for events with a query
    eventSearchWithSeries,
}

impl Display for OperationName2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationName2::recommendedEventsWithSeries => {
                write!(f, "recommendedEventsWithSeries")
            }
            OperationName2::eventSearchWithSeries => write!(f, "eventSearchWithSeries"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    pub persisted_query: PersistedQuery,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistedQuery {
    pub sha256_hash: String,
    pub version: i32,
}

// impl Default for PersistedQuery {
//     fn default() -> Self {
//         Self {
//             sha256_hash: "0aceed81313ebba814c0feadeda32f404147996091b6b77209353e2183b2dabb"
//                 .to_string(),
//             version: 1,
//         }
//     }
// }
