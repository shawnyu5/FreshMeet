use super::{
    common::{EventType, Extensions, OperationName},
    post,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// request body for getting suggested events
#[allow(non_camel_case_types, non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetYourEventsSuggestedEventsRequest {
    pub extensions: Extensions,
    pub operationName: String,
    pub variables: Variables,
}

impl Default for GetYourEventsSuggestedEventsRequest {
    fn default() -> Self {
        Self {
            extensions: Extensions::default(),
            operationName: OperationName::getYourEventsSuggestedEvents.to_string(),
            variables: Default::default(),
        }
    }
}

impl GetYourEventsSuggestedEventsRequest {
    pub async fn search(&self) -> Result<GetYourEventsSuggestedEventsResponse> {
        let result = post::<Self, GetYourEventsSuggestedEventsResponse>(self).await;
        return result;
    }
}
#[allow(non_camel_case_types, non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Variables {
    pub first: u32,
    pub lat: f32,
    pub lon: f32,
    pub eventType: String,
}

impl Default for Variables {
    fn default() -> Self {
        Self {
            first: 30,
            lat: 43.74,
            lon: -74.42,
            eventType: EventType::physical.to_string(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetYourEventsSuggestedEventsResponse {
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
    pub count: i64,
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub end_cursor: String,
    pub start_cursor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub node: Node,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub title: String,
    pub date_time: String,
    pub end_time: String,
    pub duration: String,
    pub going: i64,
    pub max_tickets: i64,
    pub timezone: String,
    pub images: Vec<Image>,
    pub event_type: String,
    pub hosts: Vec<Host>,
    pub group: Group,
    pub is_saved: bool,
    pub event_url: String,
    pub is_attending: Option<bool>,
    pub rsvp_state: String,
    pub venue: Venue,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub id: String,
    pub base_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub id: String,
    pub name: String,
    pub email: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: String,
    pub name: String,
    pub is_organizer: bool,
    pub link: String,
    pub is_private: bool,
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Venue {
    pub city: String,
    pub state: String,
}
