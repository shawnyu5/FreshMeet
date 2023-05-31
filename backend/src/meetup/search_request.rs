use super::search_types::*;
use chrono::{DateTime, Utc};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
/// request body for meetup search
pub struct RequestBody {
    pub operationName: String,
    pub variables: Variables,
    pub query: String,
}

impl Default for RequestBody {
    #[allow(dead_code)]
    fn default() -> RequestBody {
        let utc: DateTime<Utc> = Utc::now();
        let start_date_range = utc.format("%Y-%m-%dT%H:%M:%S-05:00[US/Eastern]");
        return RequestBody {
                operationName: "eventKeywordSearch".to_string(),
                variables: Variables {
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

impl RequestBody {
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
                return Ok(search);
            }
            Err(e) => {
                return Err(format!("error: {}", e));
            }
        }
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

/// construct a request object
///
/// * `query`: the search query
/// * `event_type`: the event type
/// * `first`: number of results to return
#[derive(Default, Debug, PartialEq)]
struct RequestBuilder {
    pub query: String,
    pub event_type: EventType,
    pub first: i32,
}

impl RequestBuilder {
    pub fn new() -> RequestBuilder {
        return RequestBuilder::default();
    }

    pub fn query(&mut self, query: &str) -> &mut RequestBuilder {
        self.query = query.to_string();
        return self;
    }

    pub fn event_type(&mut self, event_type: EventType) -> &mut RequestBuilder {
        self.event_type = event_type;
        return self;
    }

    pub fn first(&mut self, first: i32) -> &mut RequestBuilder {
        self.first = first;
        return self;
    }

    /// build the request body
    pub fn build(&self) -> RequestBody {
        return RequestBody {
            variables: Variables {
                query: self.query.clone(),
                eventType: Some(self.event_type.clone()),
                first: self.first,
                ..Default::default()
            },
            ..Default::default()
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// test the request builder is able to construct it self
    #[test]
    fn test_builder_can_construct() {
        let builder = RequestBuilder::new();
        assert_eq!(builder, RequestBuilder::default());
    }

    /// test the builder can build a request and set the correct values in the build request
    #[test]
    fn test_builder_can_build() {
        let mut builder = RequestBuilder::new();
        builder.query("tech meetups");
        builder.event_type(EventType::physical);
        builder.first(10);

        let request = builder.build();
        assert_eq!(request.variables.query, "tech meetups".to_string());
        assert_eq!(request.variables.eventType, Some(EventType::physical));
        assert_eq!(request.variables.first, 10);
    }

    #[tokio::test]
    async fn test_search_pysical_events() {
        // let search = Search::default();
        let mut request = RequestBody::default();
        request.variables.query = "tech meetups".to_string();
        request.variables.eventType = Some(EventType::physical);
        request.variables.first = 10;
        let result = request.search().await.unwrap();
        assert_eq!(result.data.results.count, 10);
        assert_eq!(result.data.results.edges.len(), 10);
    }

    #[tokio::test]
    async fn test_search_pagination() {
        let mut request = RequestBody::default();
        request.variables.query = "tech meetups".to_string();
        request.variables.eventType = Some(EventType::physical);
        request.variables.first = 10;
        let page_1 = request.search().await.unwrap();

        let mut request = RequestBody::default();
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
