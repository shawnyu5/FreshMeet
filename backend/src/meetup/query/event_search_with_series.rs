//! Search for events, return events including series

use crate::meetup::request_builder::Builder;

use super::{
    common::{EventType, OperationName},
    request::gql2::{SearchRequest, Variables},
};

#[derive(Debug, Default)]
pub struct SearchEventsWithSeries {
    /// After cursor
    after: Option<String>,
    /// Search string
    query: String,
    /// Type of event
    event_type: EventType,
}

impl SearchEventsWithSeries {
    pub fn event_type(&mut self, event_type: EventType) -> &mut Self {
        self.event_type = event_type;
        return self;
    }

    pub fn query(&mut self, query: String) -> &mut Self {
        self.query = query;
        return self;
    }

    pub fn after(&mut self, after: Option<String>) -> &mut Self {
        self.after = after;
        return self;
    }
}

impl Builder<SearchRequest> for SearchEventsWithSeries {
    fn new() -> Self {
        return SearchEventsWithSeries::default();
    }

    fn build(&mut self) -> SearchRequest {
        return SearchRequest {
            variables: Variables {
                query: Some(self.query.clone()),
                after: self.after.clone(),
                event_type: self.event_type.to_string(),
                ..Default::default()
            },
            operation_name: OperationName::eventSearchWithSeries.to_string(),
            ..Default::default()
        };
    }
}
