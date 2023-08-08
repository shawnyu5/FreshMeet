use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
/// request body for meetup search
pub struct RequestBody {
    /// the query to search for
    query: String,
    /// number of results to return per page
    per_page: i32,
}
/// handles /meetup/search post route
pub async fn search(Json(body): Json<RequestBody>) -> String {
    "meetup".to_string()
}

#[cfg(test)]
mod tests {
    use crate::routes::app;
    use axum::{
        body::Body,
        http::{self, Request},
    };
    use tower::ServiceExt; // for `oneshot` and `ready`

    #[tokio::test]
    async fn hello() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/meetup/search")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), http::StatusCode::OK);
    }
}
