//! Get recommended events, including event series
use super::request::gql2::{SearchRequest, Variables};
use crate::{
    meetup::request_builder::Builder,
    utils::{eod, now},
};

#[derive(Debug, Default)]
pub struct RecommendedEventsWithSeries {
    /// The after cursor
    after: Option<String>,
    /// Number of results to return
    first: Option<i32>,
    // start_date_range: Option<String>,
    // end_date_range: String,
}

impl RecommendedEventsWithSeries {
    /// Set the after cursor
    ///
    /// * `after`: after cursor to set
    pub fn after(&mut self, after: Option<String>) -> &mut Self {
        self.after = after;
        return self;
    }

    /// Set number of results to return
    ///
    /// * `first`: number of events to return
    pub fn per_page(&mut self, first: i32) -> &mut Self {
        self.first = Some(first);
        return self;
    }

    // /// Get start date range
    // ///
    // /// * `start_date_range`: start date of event to return
    // pub fn start_date_range(&mut self, start_date_range: String) -> &mut Self {
    //     self.start_date_range = Some(start_date_range);
    //     return self;
    // }
}

impl Builder<SearchRequest> for RecommendedEventsWithSeries {
    fn new() -> Self {
        return RecommendedEventsWithSeries::default();
    }

    /// constructs a search request
    fn build(&mut self) -> SearchRequest {
        return SearchRequest {
            variables: Variables {
                after: self.after.clone(),
                first: self.first.unwrap_or(30),
                // Today's date
                start_date_range: now(),
                // End of today
                end_date_range: Some(eod()),
                ..Default::default()
            },
            ..Default::default()
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_create() {
        let builder = RecommendedEventsWithSeries::new();
        dbg!(builder);
    }

    #[test]
    fn can_build() {
        let request = RecommendedEventsWithSeries::new().build();
        assert_eq!(request.operation_name, "categorySearch");
    }

    #[test]
    fn can_set_after() {
        let request = RecommendedEventsWithSeries::new()
            .after(Some("test".to_string()))
            .build();
        assert_eq!(request.variables.after, Some("test".to_string()));
    }

    #[test]
    fn can_set_first() {
        let request = RecommendedEventsWithSeries::new().per_page(22).build();
        assert_eq!(request.variables.first, 22);
    }
}
