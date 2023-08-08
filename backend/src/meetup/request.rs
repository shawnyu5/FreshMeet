use anyhow::{anyhow, Result};
use axum::http::{HeaderMap, HeaderValue};
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::response::SearchResponse;

/// Builder for building a meetup request
#[derive(Default, Debug)]
pub struct RequestBuilder {
    pub query: String,
    pub first: i32,
    pub after: Option<String>,
}

impl RequestBuilder {
    /// construct a new request builder
    pub fn new() -> RequestBuilder {
        return RequestBuilder::default();
    }

    /// set the query to search for
    pub fn query(&mut self, query: &str) -> &mut RequestBuilder {
        self.query = query.to_string();
        return self;
    }

    /// number of results to return
    pub fn per_page(&mut self, per_page: i32) -> &mut RequestBuilder {
        self.first = per_page;
        return self;
    }

    /// set the after cursor
    pub fn after(&mut self, after: Option<String>) -> &mut RequestBuilder {
        self.after = after;
        return self;
    }

    /// build the request body
    pub fn build(&self) -> RequestBody {
        return RequestBody {
            variables: Variables {
                query: self.query.clone(),
                first: self.first,
                after: self.after.clone(),
                ..Default::default()
            },
            ..Default::default()
        };
    }
}

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

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
/// request body for meetup search
pub struct RequestBody {
    pub operationName: String,
    pub variables: Variables,
    pub query: String,
}

impl RequestBody {
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

impl Default for RequestBody {
    #[allow(dead_code)]
    fn default() -> RequestBody {
        return RequestBody {
                operationName: "eventKeywordSearch".to_string(),
                variables: Default::default(),
                query:  "query eventKeywordSearch($first: Int, $after: String, $query: String!, $lat: Float!, $lon: Float!, $startDateRange: ZonedDateTime, $endDateRange: ZonedDateTime, $eventType: EventType, $radius: Int, $source: [SearchSources!]!, $isHappeningNow: Boolean, $isStartingSoon: Boolean, $categoryId: Int, $topicCategoryId: Int, $city: String, $state: String, $country: String, $zip: String, $sortField: KeywordSortField) {\n  results: keywordSearch(\n    input: {first: $first, after: $after}\n    filter: {query: $query, lat: $lat, lon: $lon, source: $source, startDateRange: $startDateRange, endDateRange: $endDateRange, eventType: $eventType, radius: $radius, isHappeningNow: $isHappeningNow, isStartingSoon: $isStartingSoon, categoryId: $categoryId, topicCategoryId: $topicCategoryId, city: $city, state: $state, country: $country, zip: $zip}\n    sort: {sortField: $sortField}\n  ) {\n    pageInfo {\n      ...PageInfoDetails\n      __typename\n    }\n    count\n    edges {\n      node {\n        id\n        result {\n          ... on Event {\n            isNewGroup\n            ...BuildMeetupEvent\n            covidPrecautions {\n              venueType\n              __typename\n            }\n            __typename\n          }\n          __typename\n        }\n        __typename\n      }\n      recommendationSource\n      recommendationId\n      __typename\n    }\n    __typename\n  }\n}\n\nfragment PageInfoDetails on PageInfo {\n  hasNextPage\n  endCursor\n  __typename\n}\n\nfragment BuildMeetupEvent on Event {\n  id\n  title\n  dateTime\n  endTime\n  description\n  duration\n  timezone\n  eventType\n  currency\n  images {\n    ...PhotoDetails\n    __typename\n  }\n  venue {\n    id\n    address\n    neighborhood\n    city\n    state\n    country\n    lat\n    lng\n    zoom\n    name\n    radius\n    __typename\n  }\n  onlineVenue {\n    type\n    url\n    __typename\n  }\n  isSaved\n  eventUrl\n  group {\n    ...BuildMeetupGroup\n    __typename\n  }\n  going\n  maxTickets\n  tickets(input: {first: 3}) {\n    ...TicketsConnection\n    __typename\n  }\n  isAttending\n  rsvpState\n  __typename\n}\n\nfragment PhotoDetails on Image {\n  id\n  baseUrl\n  preview\n  source\n  __typename\n}\n\nfragment BuildMeetupGroup on Group {\n  id\n  slug\n  isPrivate\n  isOrganizer\n  isNewGroup\n  ...GroupDetails\n  __typename\n}\n\nfragment GroupDetails on Group {\n  id\n  name\n  urlname\n  timezone\n  link\n  city\n  state\n  country\n  groupPhoto {\n    ...PhotoDetails\n    __typename\n  }\n  __typename\n}\n\nfragment TicketsConnection on EventTicketsConnection {\n  count\n  edges {\n    node {\n      id\n      user {\n        id\n        name\n        memberPhoto {\n          ...PhotoDetails\n          __typename\n        }\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n".to_string(),
            };
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
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
    pub query: String,
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
            query: "".to_string(),
            sortField: "RELEVANCE".to_string(),
            city: "Toronto".to_string(),
            state: "ON".to_string(),
            country: "ca".to_string(),
            zip: "M5M3M2".to_string(),
        }
    }
}