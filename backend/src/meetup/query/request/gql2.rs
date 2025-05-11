//! Types for meetup GQL2 API
use std::cmp::Ordering;
use std::fmt::Display;

use crate::meetup::query::common::EventType;
use crate::meetup::query::common::{Extensions, OperationName2, PersistedQuery};
use crate::utils::now;
use anyhow::anyhow;
use anyhow::Result;
use bon::bon;
use chrono::{Date, DateTime, Utc};
use markdown::to_html;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use tracing::{debug, error};
use urlencoding::encode;
use utoipa::ToSchema;

use super::post;

/// Represents the body of an API request to the Meetup graphql API
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub operation_name: String,
    pub extensions: Extensions,
    pub variables: Variables,
}

#[bon]
impl SearchRequest {
    #[builder]
    pub fn new(
        /// The operation name of this request
        operation_name: OperationName2,
        /// Variables of this request
        ///
        /// They configure values such as the search query, event start, end date, etc...
        variables: Option<Variables>,
    ) -> Self {
        let hash = match operation_name {
            OperationName2::recommendedEventsWithSeries => {
                "178a5949877be0a10fe15215ac7a63af505ebf5da05fd28440c4ad5060302ad2"
            }
            OperationName2::eventSearchWithSeries => {
                "b98fc059f4379053221befe6b201591ba98e3a8b06c9ede0b3c129c3b605d7c4"
            }
            OperationName2::getMyRsvps => {
                "76b2a1649b097ad05cecfff87cc3b038db1f69275129d6e8ad43bc9adbce67f8"
            }
        };

        debug!("Request variables: {:#?}", variables);
        return Self {
            operation_name: operation_name.to_string(),
            extensions: Extensions {
                persisted_query: PersistedQuery {
                    sha256_hash: hash.into(),
                    version: 1,
                },
            },
            variables: variables.unwrap_or_default(),
        };
    }
}

impl Default for Variables {
    fn default() -> Self {
        Self {
            first: 40,
            lat: 43.7400016784668,
            lon: -79.36000061035156,
            city: "Toronto".into(),
            sort_field: "RELEVANCE".into(),
            start_date_range: now(),
            end_date_range: None,
            series_start_date: Utc::now().format("%Y-%m-%d").to_string(),
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
    pub async fn fetch(&self) -> Result<GQLResponse> {
        if self.operation_name == OperationName2::eventSearchWithSeries.to_string()
            && self.variables.query.is_none()
        {
            error!(
                "When operation name is {operation_name}, a query must be included.",
                operation_name = self.operation_name
            );
            return Err(anyhow!("Missing query"));
        }
        let response = post::<SearchRequest, GQLResponse>(self).await?;
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

impl Default for SearchRequest {
    fn default() -> Self {
        return Self {
            extensions: Default::default(),
            operation_name: OperationName2::recommendedEventsWithSeries.to_string(),
            variables: Variables::default(),
        };
    }
}

impl Default for Extensions {
    fn default() -> Self {
        Self {
            persisted_query: PersistedQuery {
                sha256_hash: "178a5949877be0a10fe15215ac7a63af505ebf5da05fd28440c4ad5060302ad2"
                    .to_string(),
                version: 1,
            },
        }
    }
}

impl Edge {
    /// Formats the event start date to a more human readable format
    pub fn format_start_date(&mut self) {
        // let date = DateTime::parse_from_rfc3339(&edge.node.date_time)
        let date = DateTime::parse_from_rfc3339(&self.node.date_time)
            .expect("Failed to parse meetup start date time");
        self.node.date_time = date.format("%a %m-%d %I:%M%P").to_string();
    }

    /// Parses the event descriptions as markdown
    pub fn description_to_html(&mut self) {
        let html = to_html(self.node.description.as_str()).clone();
        self.node.description = html;
    }

    /// Populate `self.is_attending_str` based on `self.is_attending`
    pub fn is_attending_to_str(&mut self) {
        // 🔖
        let book_mark: &str = if self.node.is_saved { "🔖" } else { "" };
        if self.node.is_attending {
            self.node.is_attending_str = Some(format!("{book_mark}Attending! 😀"));
        } else {
            self.node.is_attending_str = Some(format!("{book_mark}Not attending... 🫠"));
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Variables {
    /// Number of results to return
    /// 200 is the max number of results to return
    pub first: i32,
    pub lat: f64,
    pub lon: f64,
    pub sort_field: String,
    pub start_date_range: String,
    pub end_date_range: Option<String>,
    pub series_start_date: String,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GQLResponse {
    pub data: Option<GQLData>,
    // "{\"errors\":[{\"message\":\"PersistedQueryNotFound\",\"locations\":[],\"extensions\":{\"persistedQueryId\":\"0f0332e9a4b01456580c1f669f26edc053d5
    // 0382b3e338d5ca580f194a27feab\",\"generatedBy\":\"graphql-java\",\"classification\":\"PersistedQueryNotFound\"}}],\"data\":null}"
    pub errors: Option<Vec<Value>>,
}

impl GQLResponse {
    pub fn sort(&mut self) {
        self.sort_by_start_date();
        self.sort_by_is_saved();
        self.sort_by_is_attending();
    }
    /// Sort the meetups with events starting the soonest first
    fn sort_by_start_date(&mut self) {
        self.data.as_mut().unwrap().result.edges.sort_by(|a, b| {
            let a_date = DateTime::parse_from_rfc3339(&a.node.date_time)
                .expect("Failed to parse meetup start date time");
            let b_date = DateTime::parse_from_rfc3339(&b.node.date_time)
                .expect("Failed to parse meetup start date time");

            if a_date > b_date {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
    }

    /// Sort events by placing saved events first
    fn sort_by_is_saved(&mut self) {
        self.data.as_mut().unwrap().result.edges.sort_by(|a, _| {
            if a.node.is_saved {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        });
    }

    /// Sort events by placing events that is attending first
    fn sort_by_is_attending(&mut self) {
        self.data.as_mut().unwrap().result.edges.sort_by(|a, _| {
            if a.node.is_attending {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        });
    }

    /// Format all events by doing the following:
    /// - Compiles the description of the event to HTML
    /// - Formats the starting date of the meetup in a human readable format
    /// - Populates `is_attending_str` for all events
    pub fn format(&mut self) {
        self.data
            .as_mut()
            .unwrap()
            .result
            .edges
            .par_iter_mut()
            .map(|edge| {
                edge.description_to_html();
                edge.format_start_date();
                edge.is_attending_to_str();
            })
            .for_each(drop);
    }

    pub fn generate_google_maps_url(&mut self) {
        self.data
            .as_mut()
            .unwrap()
            .result
            .edges
            .par_iter_mut()
            .map(|edge| {
                if edge.node.venue.is_some() {
                    edge.node.google_maps_url = Some(format!(
                        "https://www.google.com/maps/dir/?api=1&destination={dest}",
                        dest = encode(&edge.node.venue.clone().unwrap_or_default().to_string())
                    ))
                }
            })
            .for_each(drop);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GQLData {
    #[serde(alias = "results")]
    pub result: MeetupResult,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MeetupResult {
    pub page_info: PageInfo,
    pub total_count: i64,
    pub edges: Vec<Edge>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub node: Node,
    pub metadata: Metadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub date_time: String,
    pub google_maps_url: Option<String>,
    pub description: String,
    pub event_type: String,
    pub event_url: String,
    pub featured_event_photo: Option<FeaturedEventPhoto>,
    pub fee_settings: Option<FeeSettings>,
    pub id: String,
    pub is_attending: bool,
    /// A string description of if this event will be attended or not
    pub is_attending_str: Option<String>,
    pub is_online: bool,
    pub is_saved: bool,
    pub covid_precautions: CovidPrecautions,
    pub group: Group,
    pub max_tickets: i64,
    pub rsvps: Rsvps,
    pub title: String,
    pub venue: Option<Venue>,
    pub social_labels: Vec<Value>,
    pub rsvp_state: String,
    pub series: Option<Series>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FeaturedEventPhoto {
    pub base_url: String,
    pub high_res_url: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FeeSettings {
    pub accepts: String,
    pub currency: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CovidPrecautions {
    pub venue_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MembershipMetadata {
    pub role: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeyGroupPhoto {
    pub base_url: String,
    pub high_res_url: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Rsvps {
    pub total_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Venue {
    pub id: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub city: String,
    pub state: String,
    pub country: String,
}

impl Display for Venue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{lat},{lon}", lat = self.lat, lon = self.lon);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    pub events: Events,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Events {
    pub edges: Vec<Edge2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Edge2 {
    pub node: Node2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Node2 {
    pub id: String,
    pub date_time: String,
    pub is_attending: bool,
    pub group: Group2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Group2 {
    pub urlname: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub rec_id: String,
    pub rec_source: String,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    /// Validate we can build a simple request
    fn can_build_request() {
        let _ = SearchRequest::builder()
            .operation_name(OperationName2::recommendedEventsWithSeries)
            .build();
    }
}
