use anyhow::{anyhow, Result};
use bon::bon;
use serde::{Deserialize, Serialize};

use crate::{
    meetup::query::common::{EventType, OperationName2},
    utils::now,
};

use super::{gql2::GQLResponse, post};

/// Represents the body of an API request to the Meetup graphql API
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub operation_name: String,
    pub extensions: Extensions,
    pub variables: T,
}

#[bon]
impl SearchRequest<SearchVariables> {
    /// Builds a request for requesting recommended events
    #[builder(finish_fn(name = build))]
    pub fn recommended_events_with_series(
        // TODO: should narrow down this type, to smth more specific, other than Variables
        /// Variables of this request
        ///
        /// They configure values such as the search query, event start, end date, etc...
        variables: SearchVariables,
    ) -> Self {
        return SearchRequest::<SearchVariables> {
            operation_name: OperationName2::recommendedEventsWithSeries.to_string(),
            extensions: Extensions {
                persisted_query: PersistedQuery {
                    sha256_hash: "d3b3542df9c417007a7e6083b931d2ed67073f4d74891c3f14da403164e56469"
                        .to_string(),
                    version: 1,
                },
            },
            variables,
        };
    }

    /// Builds a request for searching events
    #[builder(finish_fn(name = build))]
    pub fn event_search_with_series(
        /// The search query
        query: &str,
        /// Variables of this request
        ///
        /// They configure values such as the search query, event start, end date, etc...
        variables: SearchVariables,
    ) -> Self {
        return SearchRequest::<SearchVariables> {
            operation_name: OperationName2::recommendedEventsWithSeries.to_string(),
            extensions: Extensions {
                persisted_query: PersistedQuery {
                    sha256_hash: "b98fc059f4379053221befe6b201591ba98e3a8b06c9ede0b3c129c3b605d7c4"
                        .to_string(),
                    version: 1,
                },
            },
            variables: SearchVariables {
                query: Some(query.to_string()),
                ..variables
            },
        };
    }
}

#[bon]
impl SearchRequest<RsvpVariables> {
    #[builder]
    /// Builds a search request for getting RSVPed events
    ///
    /// * `start_date`: start date of events. Defaults to today
    /// * `first`: number of events to fetch. Defaults to 10
    pub fn new(start_date: Option<&str>, first: Option<i32>) -> Self {
        return Self {
            operation_name: OperationName2::getMyRsvps.to_string(),
            extensions: Extensions {
                persisted_query: PersistedQuery {
                    sha256_hash: "76b2a1649b097ad05cecfff87cc3b038db1f69275129d6e8ad43bc9adbce67f8"
                        .to_string(),
                    version: 1,
                },
            },
            variables: RsvpVariables {
                start_date: start_date.unwrap_or_default().to_string(),
                first: first.unwrap_or_default(),
                ..Default::default()
            },
        };
    }
}

impl<T> SearchRequest<T>
where
    T: Serialize + for<'de> Deserialize<'de> + std::fmt::Debug,
{
    /// Send the API request
    pub async fn fetch(&self) -> Result<GQLResponse> {
        let response = post::<Self, GQLResponse>(self).await?;
        // If we get data back, then the request is successful
        // If not data, then return the error message. Something went wrong...
        if response.data.is_some() {
            return Ok(response);
        } else {
            return Err(anyhow!(
                serde_json::to_string(&response.errors).unwrap_or_default()
            ));
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RsvpVariables {
    pub start_date: String,
    pub after: Option<String>,
    pub first: i32,
    pub event_status: Vec<String>,
    pub rsvp_status: Vec<String>,
    pub extensions: Extensions,
}

impl Default for RsvpVariables {
    fn default() -> Self {
        Self {
            start_date: now(),
            after: None,
            first: 10,
            event_status: vec!["UPCOMING".to_string()],
            rsvp_status: vec!["YES".to_string(), "WAITLIST".to_string()],
            extensions: Extensions {
                persisted_query: PersistedQuery {
                    sha256_hash: "76b2a1649b097ad05cecfff87cc3b038db1f69275129d6e8ad43bc9adbce67f8"
                        .to_string(),
                    version: 1,
                },
            },
        }
    }
}
// {"operationName":"getMyRsvps","variables":{"startDate":"2024-12-30T00:00:00-05:00","after":null,"first":20,"eventStatus":["UPCOMING"],"rsvpStatus":["YES","WAITLIST"]},"extensions":{"persistedQuery":{"version":1,"sha256Hash":"76b2a1649b097ad05cecfff87cc3b038db1f69275129d6e8ad43bc9adbce67f8"}}}
/// Variables used for searching meetups
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchVariables {
    /// Number of results to return
    pub first: i32,
    pub lat: f64,
    pub lon: f64,
    pub sort_field: String,
    pub start_date_range: String,
    pub end_date_range: Option<String>,
    /// The after cursor
    pub after: Option<String>,
    /// Type of event
    pub event_type: String,
    pub index_alias: String,
    pub do_consolidate_events: bool,
    pub do_promote_paypal_events: bool,
    pub city: String,
    pub number_of_events_for_series: i32,
    /// Search query. Only applicable with SearchRequest operation `eventSearchWithSeries`
    pub query: Option<String>,
}

impl Default for SearchVariables {
    fn default() -> Self {
        Self {
            first: 40,
            lat: 43.7400016784668,
            lon: -79.36000061035156,
            city: "Toronto".into(),
            sort_field: "RELEVANCE".into(),
            start_date_range: now(),
            end_date_range: None,
            after: None,
            event_type: EventType::default().to_string(),
            index_alias: "popular_events_nearby_current".into(),
            do_consolidate_events: true,
            do_promote_paypal_events: false,
            number_of_events_for_series: 5,
            query: None,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_build_recommended_events_with_series_request() {
        SearchRequest::<SearchVariables>::recommended_events_with_series()
            .variables(SearchVariables::default())
            .build();
    }

    #[test]
    fn can_build_event_search_with_series_request() {
        SearchRequest::<RsvpVariables>::builder();
        SearchRequest::<SearchVariables>::event_search_with_series()
            .query("test")
            .variables(SearchVariables::default())
            .build();
    }
}
