use axum::Json;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::meetup::{
    request::{
        category_search::CategorySearchResponse,
        category_search_builder::CategorySearchRequestBuilder, common::EventType,
        event_keyword_search_builder::EventKeyWrodSearchBuilder,
        get_your_events_suggested_events::GetYourEventsSuggestedEventsResponse,
        get_your_events_suggested_events_builder::GetYourEventsSuggestedEventsBuilder,
    },
    request_builder::Builder,
    response::{Event, PageInfo, RsvpState},
};

/// request body for meetup search
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RequestBody {
    /// the query to search for
    query: String,
    /// number of results to return per page
    per_page: i32,
    /// the after cursor
    after: Option<String>,
}

/// response body for meetup search
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub page_info: PageInfo,
    pub nodes: Vec<Event>,
}
/// handles /meetup/search post route
pub async fn search(Json(body): Json<RequestBody>) -> Result<Json<Response>, StatusCode> {
    let request = EventKeyWrodSearchBuilder::new()
        .query(body.query.as_str())
        .per_page(body.per_page)
        .after(body.after)
        .build();

    let response = match request.search().await {
        Ok(r) => r,
        Err(e) => {
            dbg!(&e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    let mut events: Vec<Event> = response
        .events()
        .iter()
        .filter(|e| e.rsvpState != RsvpState::CLOSED && !e.title.to_lowercase().contains("online"))
        .cloned()
        .collect();
    events.sort_by(|a, b| a.dateTime.cmp(&b.dateTime));

    return Ok(Json(Response {
        page_info: response.data.results.pageInfo,
        nodes: events,
    }));
}

/// Handles `meetup/suggested` route. Fetches suggested events from Meetup API
pub async fn suggested_events() -> Result<Json<GetYourEventsSuggestedEventsResponse>, StatusCode> {
    let request = GetYourEventsSuggestedEventsBuilder::new()
        .event_type(EventType::physical)
        .first(40)
        .build();

    let mut response = match request.search().await {
        Ok(r) => r,
        Err(e) => {
            dbg!(&e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    response
        .data
        .ranked_events
        .edges
        .sort_by(|a, b| a.node.date_time.cmp(&b.node.date_time));

    return Ok(Json(response));
}
/// Fetch meetups for today
pub async fn meetups_today() -> Result<Json<CategorySearchResponse>, StatusCode> {
    match CategorySearchRequestBuilder::new().build().fetch().await {
        Ok(r) => Ok(Json(r)),
        Err(e) => {
            error!("Error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        meetup::request::get_your_events_suggested_events::GetYourEventsSuggestedEventsResponse,
        routes::{
            app,
            meetup::{RequestBody, Response},
        },
    };
    use axum::{
        body::Body,
        http::{self, Request},
    };
    use tower::ServiceExt; // for `oneshot` and `ready`

    #[tokio::test]
    async fn meetup_search_status_code() {
        let app = app();

        let body = RequestBody {
            query: "programming".to_string(),
            per_page: 10,
            after: None,
        };
        let json_data = serde_json::to_string(&body).unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/meetup/search")
                    .header("Content-Type", "application/json")
                    .body(Body::from(json_data))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), http::StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Response = serde_json::from_slice(&body).unwrap();
        assert_eq!(body.nodes.len(), 10 as usize);
    }

    /// test all meetup search result titles does not contain `online`
    #[tokio::test]
    async fn no_online_in_search_result_title() {
        let app = app();

        let body = RequestBody {
            query: "dating".to_string(),
            per_page: 20,
            after: None,
        };
        let json_data = serde_json::to_string(&body).unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/meetup/search")
                    .header("Content-Type", "application/json")
                    .body(Body::from(json_data))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), http::StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Response = serde_json::from_slice(&body).unwrap();
        body.nodes.iter().for_each(|e| {
            assert_eq!(e.title.contains("online"), false);
        });
    }

    /// Make sure suggested events route returns events
    #[tokio::test]
    async fn can_get_suggested_events() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/meetup/suggested")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), http::StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: GetYourEventsSuggestedEventsResponse = serde_json::from_slice(&body).unwrap();

        assert_ne!(body.data.ranked_events.count, 0);
        assert_ne!(body.data.ranked_events.edges.len(), 0);
    }

    /// test `/meetup/today` route returns successful status code
    #[tokio::test]
    async fn can_get_meetups_today() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/meetup/today")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }
}
