use axum::Json;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::meetup::{
    request::RequestBuilder,
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
    dbg!(&body);
    let request = RequestBuilder::new()
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
        .filter(|e| e.rsvpState != RsvpState::CLOSED)
        .cloned()
        .collect();
    events.sort_by(|a, b| a.dateTime.cmp(&b.dateTime));

    return Ok(Json(Response {
        page_info: response.data.results.pageInfo,
        nodes: events,
    }));
}

#[cfg(test)]
mod tests {
    use crate::routes::{app, meetup::RequestBody};
    use axum::{
        body::Body,
        http::{self, Request},
    };
    use tower::ServiceExt; // for `oneshot` and `ready`

    #[tokio::test]
    async fn meetup() {
        let app = app();

        let body = RequestBody {
            query: "rust".to_string(),
            per_page: 10,
            after: None,
        };
        let json_data = serde_json::to_string(&body).unwrap();
        dbg!(&json_data);

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
    }
}
