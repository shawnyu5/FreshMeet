use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum EventType {
    physical,
    online,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum RsvpState {
    #[default]
    JOIN_OPEN,
    CLOSED,
    JOIN_APPROVAL,
    NOT_OPEN_YET,
}

impl Default for EventType {
    /// physical/in person is the default event type
    fn default() -> Self {
        EventType::physical
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub data: Data,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Data {
    pub results: Results,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Results {
    pub pageInfo: PageInfo,
    pub count: i32,
    pub edges: Vec<Edge>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct PageInfo {
    pub hasNextPage: bool,
    pub endCursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Edge {
    pub node: Node,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Node {
    pub id: String,
    pub result: Result_,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
/// Details about a meetup event
///
/// * `id`: id of the event
/// * `title`: title of the event
/// * `dateTime`: date of event
/// * `endTime`: end time of event
/// * `description`: description of event
/// * `duration`: duration of event
/// * `timezone`: timezone of event
/// * `eventType`: event type. Default EventType::physical
/// * `currency`: currency of event
/// * `eventUrl`: url to event details
/// * `going`: number of people going to the even`
/// * `isAttending`: whether or not the user is attending the event
/// * `rsvpState`: state of RSVP
pub struct Result_ {
    pub id: String,
    pub title: String,
    pub dateTime: String,
    pub endTime: String,
    pub description: String,
    pub duration: String,
    pub timezone: String,
    pub eventType: String,
    pub currency: String,
    pub eventUrl: String,
    pub going: Option<i32>,
    pub isAttending: bool,
    pub rsvpState: RsvpState,
}

pub mod request_body {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    use super::EventType;

    /// convert a std::time::SystemTime to a String in ISO 8601 format
    ///
    /// * `st`: a system time to be converted
    // fn iso8601(st: &std::time::SystemTime) -> String {
    // let dt: DateTime<Utc> = st.clone().into();
    // return format!("{}", dt.format("%+"));
    // // formats like "2001-07-08T00:34:60.026490+09:30"
    // }

    impl Default for super::request_body::Body {
        #[allow(dead_code)]
        fn default() -> super::request_body::Body {
            let utc: DateTime<Utc> = Utc::now();
            let start_date_range = utc.format("%Y-%m-%dT%H:%M:%S-05:00[US/Eastern]");
            return super::request_body::Body {
                operationName: "eventKeywordSearch".to_string(),
                variables: super::request_body::Variables {
                    after: "".to_string(),
                    first: 20,
                    lat: 43.7400016784668,
                    lon: -79.36000061035156,
                    topicCategoryId: None,
                    eventType: Some(EventType::physical),
                    startDateRange: start_date_range.to_string(),
                    startDate: None,
                    source: "EVENTS".to_string(),
                    query: "".to_string(),
                    sortField: "RELEVANCE".to_string(),
                    city: "Toronto".to_string(),
                    state: "ON".to_string(),
                    country: "ca".to_string(),
                    zip: "M3B 0A3".to_string(),
                },
                query:  "query eventKeywordSearch($first: Int, $after: String, $query: String!, $lat: Float!, $lon: Float!, $startDateRange: ZonedDateTime, $endDateRange: ZonedDateTime, $eventType: EventType, $radius: Int, $source: [SearchSources!]!, $isHappeningNow: Boolean, $isStartingSoon: Boolean, $categoryId: Int, $topicCategoryId: Int, $city: String, $state: String, $country: String, $zip: String, $sortField: KeywordSortField) {\n  results: keywordSearch(\n    input: {first: $first, after: $after}\n    filter: {query: $query, lat: $lat, lon: $lon, source: $source, startDateRange: $startDateRange, endDateRange: $endDateRange, eventType: $eventType, radius: $radius, isHappeningNow: $isHappeningNow, isStartingSoon: $isStartingSoon, categoryId: $categoryId, topicCategoryId: $topicCategoryId, city: $city, state: $state, country: $country, zip: $zip}\n    sort: {sortField: $sortField}\n  ) {\n    pageInfo {\n      ...PageInfoDetails\n      __typename\n    }\n    count\n    edges {\n      node {\n        id\n        result {\n          ... on Event {\n            isNewGroup\n            ...BuildMeetupEvent\n            covidPrecautions {\n              venueType\n              __typename\n            }\n            __typename\n          }\n          __typename\n        }\n        __typename\n      }\n      recommendationSource\n      recommendationId\n      __typename\n    }\n    __typename\n  }\n}\n\nfragment PageInfoDetails on PageInfo {\n  hasNextPage\n  endCursor\n  __typename\n}\n\nfragment BuildMeetupEvent on Event {\n  id\n  title\n  dateTime\n  endTime\n  description\n  duration\n  timezone\n  eventType\n  currency\n  images {\n    ...PhotoDetails\n    __typename\n  }\n  venue {\n    id\n    address\n    neighborhood\n    city\n    state\n    country\n    lat\n    lng\n    zoom\n    name\n    radius\n    __typename\n  }\n  onlineVenue {\n    type\n    url\n    __typename\n  }\n  isSaved\n  eventUrl\n  group {\n    ...BuildMeetupGroup\n    __typename\n  }\n  going\n  maxTickets\n  tickets(input: {first: 3}) {\n    ...TicketsConnection\n    __typename\n  }\n  isAttending\n  rsvpState\n  __typename\n}\n\nfragment PhotoDetails on Image {\n  id\n  baseUrl\n  preview\n  source\n  __typename\n}\n\nfragment BuildMeetupGroup on Group {\n  id\n  slug\n  isPrivate\n  isOrganizer\n  isNewGroup\n  ...GroupDetails\n  __typename\n}\n\nfragment GroupDetails on Group {\n  id\n  name\n  urlname\n  timezone\n  link\n  city\n  state\n  country\n  groupPhoto {\n    ...PhotoDetails\n    __typename\n  }\n  __typename\n}\n\nfragment TicketsConnection on EventTicketsConnection {\n  count\n  edges {\n    node {\n      id\n      user {\n        id\n        name\n        memberPhoto {\n          ...PhotoDetails\n          __typename\n        }\n        __typename\n      }\n      __typename\n    }\n    __typename\n  }\n  __typename\n}\n".to_string(),
            };
        }
    }
    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Body {
        pub operationName: String,
        pub variables: Variables,
        pub query: String,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Variables {
        pub after: String,
        pub first: i32,
        pub lat: f64,
        pub lon: f64,
        pub eventType: Option<super::EventType>,
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

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Extensions {
        pub persistedQuery: PersistedQuery,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct PersistedQuery {
        version: i32,
        sha256Hash: String,
    }
}
impl Default for SearchResult {
    fn default() -> SearchResult {
        return SearchResult {
            data: Data {
                results: Results {
                    pageInfo: PageInfo {
                        hasNextPage: false,
                        endCursor: None,
                    },
                    count: 0,
                    edges: vec![],
                },
            },
        };
    }
}

impl request_body::Body {
    /// search for meetup events
    pub async fn search(&self) -> Result<SearchResult, String> {
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
            .json::<SearchResult>()
            .await
        {
            Ok(search) => {
                return {
                    // dbg!(&search);
                    Ok(search)
                };
            }
            Err(e) => {
                return Err(format!("error: {}", e));
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_search_pysical_events() {
        // let search = Search::default();
        let mut request = request_body::Body::default();
        request.variables.query = "tech meetups".to_string();
        request.variables.eventType = Some(EventType::physical);
        request.variables.first = 10;
        let result = request.search().await.unwrap();
        assert_eq!(result.data.results.count, 10);
        assert_eq!(result.data.results.edges.len(), 10);
    }

    #[tokio::test]
    async fn test_search_pagination() {
        let mut request = request_body::Body::default();
        request.variables.query = "tech meetups".to_string();
        request.variables.eventType = Some(EventType::physical);
        request.variables.first = 10;
        let page_1 = request.search().await.unwrap();

        let mut request = request_body::Body::default();
        request.variables.query = "tech meetups".to_string();
        request.variables.eventType = Some(EventType::physical);
        request.variables.first = 10;
        request.variables.after = page_1.data.results.pageInfo.endCursor.clone().unwrap();
        let page_2 = request.search().await.unwrap();

        let page_1 = serde_json::to_string(&page_1).unwrap();
        let page_2 = serde_json::to_string(&page_2).unwrap();
        // page sure page 1 and 2 are not the same
        assert_ne!(page_1, page_2);
    }
}
