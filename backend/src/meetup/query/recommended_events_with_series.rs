//! Get recommended events, including event series
use bon::bon;

use super::{
    common::OperationName2,
    request::gql2::{SearchRequest, Variables},
};
use crate::utils::{eod, now};

#[derive(Debug)]
pub struct RecommendedEventsWithSeries {}

#[bon]
impl RecommendedEventsWithSeries {
    #[builder]
    pub fn new(
        /// The after cursor
        after: Option<String>,
        /// Number of results to return per page
        per_page: Option<i32>,
    ) -> SearchRequest {
        return SearchRequest {
            operation_name: OperationName2::recommendedEventsWithSeries.to_string(),
            variables: Variables {
                after: after.clone(),
                first: per_page.unwrap_or(30),
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
    /// Validate we are able to build a builder
    fn can_build() {
        RecommendedEventsWithSeries::builder().build();
    }

    // #[test]
    // #[ignore]
    // fn can_build() {
    //     let request = RecommendedEventsWithSeries::builder().build();
    //     // assert_eq!(request.operation_name, "categorySearch");
    // }

    #[test]
    #[ignore]
    fn can_set_after() {
        let request = RecommendedEventsWithSeries::builder()
            .after("test".to_string())
            .build();
        assert_eq!(request.variables.after, Some("test".to_string()));
    }

    #[test]
    #[ignore]
    fn can_set_first() {
        let request = RecommendedEventsWithSeries::builder().per_page(22).build();
        assert_eq!(request.variables.first, 22);
    }
}
