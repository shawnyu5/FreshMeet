use anyhow::{anyhow, Result};
use axum::http::{HeaderMap, HeaderValue};
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::response::SearchResponse;

/// types of events a meetup can be
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EventType {
    physical,
    online,
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::physical => write!(f, "PHYSICAL"),
            EventType::online => write!(f, "ONLINE"),
        }
    }
}

impl Default for EventType {
    /// physical/in person is the default event type
    fn default() -> Self {
        EventType::physical
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_camel_case_types)]
pub enum OperationName {
    /// search for events by keywords
    eventKeywordSearch,
    /// get suggested events
    getYourEventsSuggestedEvents,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
/// request body for meetup search
pub struct RequestBody {
    pub operationName: OperationName,
    pub variables: Variables,
    pub query: Option<String>,
}

impl RequestBody {
    /// construct a new request body for a specific request type
    pub fn new(operation_name: &OperationName) -> Self {
        let utc: DateTime<Utc> = Utc::now();
        let start_date_range = utc.format("%Y-%m-%dT%H:%M:%S-05:00[US/Eastern]");
        let today = Local::now().format("%Y-%m-%dT%H:%M:%S-05:00[US/Eastern]");

        match operation_name {
            OperationName::eventKeywordSearch => {
                let variable = Variables {
                    after: None,
                    first: 20,
                    lat: 43.7400016784668,
                    lon: -79.36000061035156,
                    topicCategoryId: None,
                    eventType: Some(EventType::physical),
                    startDateRange: start_date_range.to_string(),
                    startDate: Some(today.to_string()),
                    source: "EVENTS".to_string(),
                    query: Some("".to_string()),
                    sortField: "RELEVANCE".to_string(),
                    city: "Toronto".to_string(),
                    state: "ON".to_string(),
                    country: "ca".to_string(),
                    zip: "M5M3M2".to_string(),
                };

                return Self {
                operationName: OperationName::eventKeywordSearch,
                variables: variable,
                query:  Some("query eventKeywordSearch($first: Int, $after: String, $query: String!, $lat: Float!, $lon: Float!, $startDateRange: ZonedDateTime, $endDateRange: ZonedDateTime, $eventType: EventType, $radius: Int, $source: [SearchSources!]!, $isHappeningNow: Boolean, $isStartingSoon: Boolean, $categoryId: Int, $topicCategoryId: Int, $city: String, $state: String, $country: String, $zip: String, $sortField: KeywordSortField) {\n  results: keywordSearch(\n    input: {first: $first, after: $after}\n    filter: {query: $query, lat: $lat, lon: $lon, source: $source, startDateRange: $startDateRange, endDateRange: $endDateRange, eventType: $eventType, radius: $radius, isHappeningNow: $isHappeningNow, isStartingSoon: $isStartingSoon, categoryId: $categoryId, topicCategoryId: $topicCategoryId, city: $city, state: $state, country: $country, zip: $zip}\n    sort: {sortField: $sortField}\n  ) {\n    pageInfo {\n      ...PageInfoDetails\n      __typename\n    }\n    count\n    edges {\n      node {\n        id\n        result {\n          ... on Event {\n            isNewGroup\n            ...BuildMeetupEvent\n            covidPrecautions {\n              venueType\n              __typename\n            }\n            __typename\n          }\n          __typename\n        }\n        __typename\n      }\n      recommendationSource\n      recommendationId\n      __typename\n    }\n    __typename\n  }\n}\n\nfragment PageInfoDetails on PageInfo {\n  hasNextPage\n  endCursor\n  __typename\n}\n\nfragment BuildMeetupEvent on Event {\n  id\n  title\n  dateTime\n  endTime\n  description\n  duration\n  timezone\n  eventType\n  currency\n  images {\n    ...PhotoDetails\n    __typename\n  }\n  venue {\n    id\n    address\n    neighborhood\n    city\n    state\n    country\n    lat\n    lng\n    zoom\n    name\n    radius\n    __typename\n  }\n  onlineVenue {\n    type\n    url\n    __typename\n  }\n  isSaved\n  eventUrl\n  group {\n    ...BuildMeetupGroup\n    __typename\n  }\n  going\n  maxTickets\n  tickets(input: {first: 3}) {\n    ...TicketsConnection\n    __typename\n  }\n  isAttending\n  rsvpState\n  __typename\n}\n\nfragment PhotoDetails on Image {\n  id\n  baseUrl\n  preview\n  source\n  __typename\n}\n\nfragment BuildMeetupGroup on Group {\n  id\n  slug\n  isPrivate\n  isOrganizer\n  isNewGroup\n  ...GroupDetails\n  __typename\n}\n\nfragment GroupDetails on Group {\n  id\n  name\n  urlname\n  timezone\n  link\n  city\n  state\n  country\n  groupPhoto {\n    ...PhotoDetails\n    __typename\n  }\n  __typename\n}\n\nfragment TicketsConnection on EventTicketsConnection {\n  count\n  edges {\n    node {\n      id\n      user {\n        id\n        name\n        memberPhoto {\n          ...PhotoDetails\n          __typename\n        }\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n".to_string()),
            };
            }
            OperationName::getYourEventsSuggestedEvents => {
                let variables = Variables {
                    first: 20,
                    lat: 43.7400016784668,
                    lon: -79.36000061035156,
                    ..Default::default()
                };

                return Self {
                    operationName: OperationName::getYourEventsSuggestedEvents,
                    variables,
                    query: None,
                };
            }
        }
    }
    /// search for meetup events
    pub async fn search(&self) -> Result<SearchResponse> {
        let url = "https://www.meetup.com/gql";

        let mut headers = HeaderMap::new();
        headers.insert("content-type", HeaderValue::from_static("application/json"));

        let client = reqwest::Client::new();
        match client
            .post(url)
            .json(self)
            .headers(headers)
            .send()
            .await
            .unwrap()
            // .text()
            .json::<SearchResponse>()
            .await
        {
            Ok(search) => {
                return Ok(search);
            }
            Err(e) => {
                return Err(anyhow!(e));
            }
        }
    }
}

// impl Default for RequestBody {
//     #[allow(dead_code)]
//     fn default() -> RequestBody {
//         return RequestBody {
//                 operationName: OperationName::eventKeywordSearch,
//                 variables: Default::default(),
//                 query:  Some("query eventKeywordSearch($first: Int, $after: String, $query: String!, $lat: Float!, $lon: Float!, $startDateRange: ZonedDateTime, $endDateRange: ZonedDateTime, $eventType: EventType, $radius: Int, $source: [SearchSources!]!, $isHappeningNow: Boolean, $isStartingSoon: Boolean, $categoryId: Int, $topicCategoryId: Int, $city: String, $state: String, $country: String, $zip: String, $sortField: KeywordSortField) {\n  results: keywordSearch(\n    input: {first: $first, after: $after}\n    filter: {query: $query, lat: $lat, lon: $lon, source: $source, startDateRange: $startDateRange, endDateRange: $endDateRange, eventType: $eventType, radius: $radius, isHappeningNow: $isHappeningNow, isStartingSoon: $isStartingSoon, categoryId: $categoryId, topicCategoryId: $topicCategoryId, city: $city, state: $state, country: $country, zip: $zip}\n    sort: {sortField: $sortField}\n  ) {\n    pageInfo {\n      ...PageInfoDetails\n      __typename\n    }\n    count\n    edges {\n      node {\n        id\n        result {\n          ... on Event {\n            isNewGroup\n            ...BuildMeetupEvent\n            covidPrecautions {\n              venueType\n              __typename\n            }\n            __typename\n          }\n          __typename\n        }\n        __typename\n      }\n      recommendationSource\n      recommendationId\n      __typename\n    }\n    __typename\n  }\n}\n\nfragment PageInfoDetails on PageInfo {\n  hasNextPage\n  endCursor\n  __typename\n}\n\nfragment BuildMeetupEvent on Event {\n  id\n  title\n  dateTime\n  endTime\n  description\n  duration\n  timezone\n  eventType\n  currency\n  images {\n    ...PhotoDetails\n    __typename\n  }\n  venue {\n    id\n    address\n    neighborhood\n    city\n    state\n    country\n    lat\n    lng\n    zoom\n    name\n    radius\n    __typename\n  }\n  onlineVenue {\n    type\n    url\n    __typename\n  }\n  isSaved\n  eventUrl\n  group {\n    ...BuildMeetupGroup\n    __typename\n  }\n  going\n  maxTickets\n  tickets(input: {first: 3}) {\n    ...TicketsConnection\n    __typename\n  }\n  isAttending\n  rsvpState\n  __typename\n}\n\nfragment PhotoDetails on Image {\n  id\n  baseUrl\n  preview\n  source\n  __typename\n}\n\nfragment BuildMeetupGroup on Group {\n  id\n  slug\n  isPrivate\n  isOrganizer\n  isNewGroup\n  ...GroupDetails\n  __typename\n}\n\nfragment GroupDetails on Group {\n  id\n  name\n  urlname\n  timezone\n  link\n  city\n  state\n  country\n  groupPhoto {\n    ...PhotoDetails\n    __typename\n  }\n  __typename\n}\n\nfragment TicketsConnection on EventTicketsConnection {\n  count\n  edges {\n    node {\n      id\n      user {\n        id\n        name\n        memberPhoto {\n          ...PhotoDetails\n          __typename\n        }\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n".to_string()),
//             };
//     }
// }

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Variables {
    pub after: Option<String>,
    pub first: i32,
    pub lat: f64,
    pub lon: f64,
    pub eventType: Option<EventType>,
    pub topicCategoryId: Option<String>,
    pub startDateRange: String,
    pub startDate: Option<String>,
    pub source: String,
    pub query: Option<String>,
    pub sortField: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub zip: String,
}

impl Default for Variables {
    fn default() -> Self {
        let utc: DateTime<Utc> = Utc::now();
        let start_date_range = utc.format("%Y-%m-%dT%H:%M:%S-05:00[US/Eastern]");
        let today = Local::now().format("%Y-%m-%dT%H:%M:%S-05:00[US/Eastern]");
        Self {
            after: None,
            first: 20,
            lat: 43.7400016784668,
            lon: -79.36000061035156,
            topicCategoryId: None,
            eventType: Some(EventType::physical),
            startDateRange: start_date_range.to_string(),
            startDate: Some(today.to_string()),
            source: "EVENTS".to_string(),
            query: Some("".to_string()),
            sortField: "RELEVANCE".to_string(),
            city: "Toronto".to_string(),
            state: "ON".to_string(),
            country: "ca".to_string(),
            zip: "M5M3M2".to_string(),
        }
    }
}
