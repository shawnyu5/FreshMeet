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
