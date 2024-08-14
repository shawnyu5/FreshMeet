//! Types for meetup GQL2 API
use crate::meetup::query::common::EventType;
use crate::meetup::query::common::{Extensions, OperationName2, PersistedQuery};
use anyhow::Result as anyhow_result;
use chrono::{DateTime, FixedOffset, Utc};
use markdown::to_html;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use super::post;

/// Represents the body of an API request to the Meetup graphql API
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub operation_name: String,
    pub extensions: Extensions,
    pub variables: Variables,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Variables {
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
    /// Search query
    ///
    /// Only applicable with operation `eventSearchWithSeries`
    pub query: Option<String>,
}

impl Default for Variables {
    fn default() -> Self {
        // Get the current UTC date and time
        let utc: DateTime<Utc> = Utc::now();
        // Create a fixed offset representing -04:00 (Eastern Daylight Time)
        let offset = FixedOffset::east_opt(-4 * 3600).unwrap();
        // Convert the UTC time to the specified offset
        let local_time: DateTime<FixedOffset> = utc.with_timezone(&offset);
        // Format the local time in the desired format
        let start_time = local_time.format("%Y-%m-%dT%H:%M:%S-04:00").to_string();
        // let end_time = local_time.format("%Y-%m-%dT23:59:59-04:00").to_string();

        Self {
            first: 40,
            lat: 43.7400016784668,
            lon: -79.36000061035156,
            city: "Toronto".into(),
            sort_field: "RELEVANCE".into(),
            start_date_range: start_time,
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

impl SearchRequest {
    /// Send the API request
    pub async fn fetch(&self) -> anyhow_result<GQLResponse> {
        let response = post::<SearchRequest, GQLResponse>(self).await;
        return response;
    }
}

impl Default for SearchRequest {
    fn default() -> Self {
        return Self {
            extensions: Extensions {
                persisted_query: PersistedQuery {
                    sha256_hash: "0f0332e9a4b01456580c1f669f26edc053d50382b3e338d5ca580f194a27feab"
                        .to_string(),
                    version: 1,
                },
            },
            operation_name: OperationName2::recommendedEventsWithSeries.to_string(),
            variables: Variables::default(),
        };
    }
}

impl GQLResponse {
    /// Parses all node descriptions as markdown
    pub fn description_to_html(&mut self) {
        self.data
            .result
            .edges
            .iter_mut()
            .map(|edge| {
                let html = to_html(edge.node.description.as_str()).clone();
                edge.node.description = html;
            })
            .for_each(drop);
    }

    /// Formats all event start dates to a more human readable format
    pub fn format_start_date(&mut self) {
        self.data
            .result
            .edges
            .iter_mut()
            .map(|edge| {
                let date = DateTime::parse_from_rfc3339(&edge.node.date_time)
                    .expect("Failed to parse meetup start date time");
                edge.node.date_time = date.format("%a %m-%d %I:%M%P").to_string();
            })
            .for_each(drop);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLResponse {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub result: MeetupResult,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeetupResult {
    pub page_info: PageInfo,
    pub total_count: i64,
    pub edges: Vec<Edge>,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub node: Node,
    pub metadata: Metadata,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub date_time: String,
    pub description: String,
    pub event_type: String,
    pub event_url: String,
    pub featured_event_photo: Option<FeaturedEventPhoto>,
    pub fee_settings: Option<FeeSettings>,
    pub id: String,
    pub is_attending: bool,
    pub is_online: bool,
    pub is_saved: bool,
    pub covid_precautions: CovidPrecautions,
    pub group: Group,
    pub max_tickets: i64,
    pub rsvps: Rsvps,
    pub title: String,
    pub venue: Option<Venue>,
    pub social_labels: Vec<Value>,
    #[serde(rename = "__typename")]
    pub typename: String,
    pub rsvp_state: String,
    pub series: Option<Series>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeaturedEventPhoto {
    pub base_url: String,
    pub high_res_url: String,
    pub id: String,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeSettings {
    pub accepts: String,
    pub currency: String,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CovidPrecautions {
    pub venue_type: Option<String>,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: String,
    pub is_new_group: bool,
    pub is_private: bool,
    pub membership_metadata: Option<MembershipMetadata>,
    pub key_group_photo: Option<KeyGroupPhoto>,
    pub name: String,
    pub timezone: String,
    pub urlname: String,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MembershipMetadata {
    pub role: String,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyGroupPhoto {
    pub base_url: String,
    pub high_res_url: String,
    pub id: String,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rsvps {
    pub total_count: i64,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Venue {
    pub id: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub city: String,
    pub state: String,
    pub country: String,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    pub events: Events,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Events {
    pub edges: Vec<Edge2>,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge2 {
    pub node: Node2,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node2 {
    pub id: String,
    pub date_time: String,
    pub is_attending: bool,
    pub group: Group2,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group2 {
    pub urlname: String,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub rec_id: String,
    pub rec_source: String,
    #[serde(rename = "__typename")]
    pub typename: String,
}
