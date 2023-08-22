use crate::meetup::request_builder::Builder;

use super::{
    common::EventType,
    event_keyword_search::{EventKeywordSearchRequest, Variables},
};

#[derive(Debug, Default)]
pub struct EventKeyWrodSearchBuilder {
    query: Option<String>,
    first: i32,
    after: Option<String>,
    event_type: Option<EventType>,
}

impl Builder<EventKeywordSearchRequest> for EventKeyWrodSearchBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn build(&mut self) -> EventKeywordSearchRequest {
        return EventKeywordSearchRequest {
            variables: Variables {
                query: self.query.clone(),
                first: self.first,
                after: self.after.clone(),
                eventType: Some(
                    self.event_type
                        .as_ref()
                        .unwrap_or_else(|| &EventType::physical)
                        .clone(),
                ),
                ..Default::default()
            },
            ..Default::default()
        };
    }
}

impl EventKeyWrodSearchBuilder {
    /// set the query to search for
    pub fn query(&mut self, query: &str) -> &mut Self {
        self.query = Some(query.to_string());
        return self;
    }

    /// number of results to return
    pub fn per_page(&mut self, per_page: i32) -> &mut Self {
        self.first = per_page;
        return self;
    }

    /// set the after cursor
    pub fn after(&mut self, after: Option<String>) -> &mut Self {
        self.after = after;
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_set_query() {
        let mut builder = EventKeyWrodSearchBuilder::new();
        builder.query("tech");
        builder.per_page(10);
        let request = builder.build();

        assert_eq!(request.variables.query, Some("tech".to_string()));
    }
}
