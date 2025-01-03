/// This is a reference implementation kept here for historic purposes.
use anyhow::{anyhow, Result};
use bon::bon;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

use crate::{
    meetup::query::common::{EventType, OperationName2},
    utils::now,
};

use super::{gql2::GQLResponse, post};

/// Represents the body of an API request to the Meetup graphql API
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SearchRequest<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub operation_name: String,
    pub extensions: Extensions,
    pub variables: T,
}

/// Build request for getting recommended events
#[allow(dead_code)]
struct RecommendedEventsWithSeries {}

#[bon]
impl RecommendedEventsWithSeries {
    #[builder]
    pub fn new(
        /// Number of results to return. Defaults to 40
        first: Option<i32>,
        /// The start date. Defaults to today
        start_date: Option<String>,
        /// End date range. Defaults to end of day
        end_date: Option<String>,
    ) -> SearchRequest<SearchVariables> {
        return SearchRequest::<SearchVariables> {
            operation_name: OperationName2::recommendedEventsWithSeries.to_string(),
            extensions: Extensions {
                persisted_query: PersistedQuery {
                    sha256_hash: "d3b3542df9c417007a7e6083b931d2ed67073f4d74891c3f14da403164e56469"
                        .to_string(),
                    version: 1,
                },
            },
            variables: SearchVariables {
                first: first.unwrap_or(40),
                start_date_range: start_date.unwrap_or(now()),
                end_date_range: end_date,
                ..Default::default()
            },
        };
    }
}

/// Build request for getting RSVP events
#[allow(dead_code)]
struct RsvpEvents {}

#[bon]
impl RsvpEvents {
    /// RSVP endpoint accepts a different time format than the other endpoints. This function will convert a date time into the correct format this endpoint expects
    pub fn format_date(date: DateTime<Utc>) -> String {
        return date.format("%Y-%m-%dT00:00:00-05:00").to_string();
    }
    #[builder]
    pub fn new(
        /// Start date of event. Defaults to today
        start_date: Option<DateTime<Utc>>,
        /// Number of results to return. Defaults to 10
        first: Option<i32>,
    ) -> SearchRequest<RsvpVariables> {
        return SearchRequest::<RsvpVariables> {
            operation_name: OperationName2::getMyRsvps.to_string(),
            extensions: Extensions {
                persisted_query: PersistedQuery {
                    sha256_hash: "76b2a1649b097ad05cecfff87cc3b038db1f69275129d6e8ad43bc9adbce67f8"
                        .to_string(),
                    version: 1,
                },
            },
            variables: RsvpVariables {
                start_date: Self::format_date(start_date.unwrap_or(Utc::now())),
                first: first.unwrap_or(10),
                ..Default::default()
            },
        };
    }
}

impl<SearchVariables> SearchRequest<SearchVariables>
where
    SearchVariables: Serialize + for<'de> Deserialize<'de> + std::fmt::Debug,
{
    /// Send the API request
    pub async fn search(&self) -> Result<GQLResponse> {
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

impl<RsvpVariables> SearchRequest<RsvpVariables>
where
    RsvpVariables: Serialize + for<'de> Deserialize<'de> + std::fmt::Debug,
{
    #[allow(dead_code)]
    pub async fn rsvp(&self) -> Result<RsvpResponse> {
        let response = post::<Self, RsvpResponse>(self).await?;
        return Ok(response);
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RsvpVariables {
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
/// Variables used for searching meetups
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SearchVariables {
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
struct Extensions {
    pub persisted_query: PersistedQuery,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PersistedQuery {
    pub sha256_hash: String,
    pub version: i32,
}

/// Response from getting RSVPed events
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct RsvpResponse {
    pub data: RsvpData,
}

// impl RsvpResponse {
//     /// Converts an RSVP response to `GQLResponse`
//     pub fn to_gql_response(self) -> GQLResponse {
//         let edges = self.data.self_field.upcoming_events.edges;
//         let gql_res = GQLResponse{
//             data: Some(GQLData{
//                 result: MeetupResult{
//                     page_info: super::gql2::PageInfo{
//                         end_cursor: self.data.self_field.upcoming_events.page_info.end_cursor,
//                         has_next_page: self.data.self_field.upcoming_events.page_info.has_next_page
//                     },
//                     total_count: self.data.self_field.upcoming_events.total_count,
//                     edges:
//                 },
//             }),
//             errors: None,
//         }
//     }
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct RsvpData {
    #[serde(rename = "self")]
    pub self_field: SelfField,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct SelfField {
    pub id: String,
    pub upcoming_events: UpcomingEvents,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct UpcomingEvents {
    pub total_count: i64,
    pub page_info: PageInfo,
    pub edges: Option<Vec<Edge>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct Edge {
    pub cursor: String,
    pub node: Node,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct Node {
    pub id: String,
    pub rsvp_state: String,
    pub event: Event,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct Event {
    pub id: String,
    pub title: String,
    pub date_time: String,
    pub duration: String,
    pub end_time: String,
    pub going: Going,
    pub featured_event_photo: FeaturedEventPhoto,
    pub event_type: String,
    pub group: Group,
    pub is_saved: bool,
    pub hosts: Vec<Host>,
    pub event_url: String,
    pub is_attending: bool,
    pub max_tickets: i64,
    pub venue: Venue,
    pub social_labels: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct Going {
    pub total_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct FeaturedEventPhoto {
    pub id: String,
    pub source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct Group {
    pub id: String,
    pub name: String,
    pub urlname: String,
    pub is_primary_organizer: bool,
    pub link: String,
    pub is_private: bool,
    pub city: String,
    pub state: String,
    pub country: String,
    pub timezone: String,
    pub key_group_photo: KeyGroupPhoto,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct KeyGroupPhoto {
    pub id: String,
    pub source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct Host {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct Venue {
    pub name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub country: String,
}
