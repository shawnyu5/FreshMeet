//! Search for events that are happening today near Toronto Ontario.
use crate::meetup::request::common::OperationName;
use anyhow::Result;
use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    common::{Extensions, PersistedQuery},
    post,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySearchRequest {
    pub operation_name: String,
    pub extensions: Extensions,
    pub variables: Variables,
    pub after: Option<String>,
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
    pub end_date_range: String,
    /// The after cursor
    pub after: Option<String>,
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
        let end_time = local_time.format("%Y-%m-%dT23:59:59-04:00").to_string();

        Self {
            first: 40,
            lat: 43.7400016784668,
            lon: -79.36000061035156,
            sort_field: "RELEVANCE".to_string(),
            start_date_range: start_time,
            end_date_range: end_time,
            after: None,
        }
    }
}

impl CategorySearchRequest {
    /// Send the API request
    pub async fn fetch(&self) -> Result<CategorySearchResponse> {
        let response = post::<CategorySearchRequest, CategorySearchResponse>(&self).await;
        return response;
    }
}

impl Default for CategorySearchRequest {
    fn default() -> Self {
        return Self {
            extensions: Extensions {
                persisted_query: PersistedQuery {
                    sha256_hash: "0aceed81313ebba814c0feadeda32f404147996091b6b77209353e2183b2dabb"
                        .to_string(),
                    version: 1,
                },
            },
            operation_name: OperationName::categorySearch.to_string(),
            variables: Variables::default(),
            after: None,
        };
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategorySearchResponse {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub ranked_events: RankedEvents,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankedEvents {
    pub page_info: PageInfo,
    pub count: i64,
    pub edges: Vec<Edge>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub node: Node,
    pub recommendation_id: String,
    pub recommendation_source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub title: String,
    pub date_time: String,
    pub end_time: String,
    pub description: String,
    pub duration: String,
    pub timezone: String,
    pub event_type: String,
    pub currency: String,
    pub images: Vec<Image>,
    pub venue: Venue,
    pub online_venue: Option<OnlineVenue>,
    pub is_saved: bool,
    pub event_url: String,
    pub group: Group,
    pub going: i64,
    pub max_tickets: i64,
    pub tickets: Tickets,
    pub is_attending: Value,
    pub rsvp_state: String,
    pub is_new_group: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub id: String,
    pub base_url: String,
    pub source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Venue {
    pub id: String,
    pub address: String,
    pub neighborhood: Value,
    pub city: String,
    pub state: String,
    pub country: String,
    pub lat: f64,
    pub lng: f64,
    pub zoom: i64,
    pub name: String,
    pub radius: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnlineVenue {
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: String,
    pub slug: String,
    pub is_private: bool,
    pub is_organizer: bool,
    pub is_new_group: bool,
    pub name: String,
    pub urlname: String,
    pub timezone: String,
    pub link: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub group_photo: GroupPhoto,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupPhoto {
    pub id: String,
    pub base_url: String,
    pub source: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tickets {
    pub count: i64,
    pub edges: Vec<Edge2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge2 {
    pub node: Node2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node2 {
    pub id: String,
}
