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

impl Default for EventType {
    /// physical/in person is the default event type
    fn default() -> Self {
        EventType::physical
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
/// request body for meetup search
pub struct RequestBody {
    pub operationName: String,
    pub variables: Variables,
    pub query: String,
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
            zip: "M5M3M2".to_string(),
        }
    }
}
