//! Route handlers for meetups

use std::cmp::Ordering;

use crate::meetup::query::common::{Extensions, OperationName, OperationName2, PersistedQuery};
use crate::meetup::query::request::gql2::Variables;
use crate::meetup::query::request::gql2::{GQLResponse, SearchRequest};
use crate::utils::{eod, now};

use axum::{extract::Query, Json};

use chrono::DateTime;
use reqwest::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

use crate::meetup::{
    request_builder::Builder,
    response::{Event, PageInfo},
};

use super::AppError;

/// response body for meetup search
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub page_info: PageInfo,
    pub nodes: Vec<Event>,
}

/// Query parameters for `/today` route
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MeetupsTodayQueryParams {
    pub after: Option<String>,
}

/// Handles `/today` route
/// Get meetups for today
pub async fn meetups_today_handler(
    query: Query<MeetupsTodayQueryParams>,
) -> Result<Json<GQLResponse>, AppError> {
    match SearchRequest::builder()
        .operation_name(OperationName2::recommendedEventsWithSeries)
        .variables(Variables {
            first: 100,
            after: query.after.clone(),
            start_date_range: now(),
            end_date_range: Some(eod()),
            ..Default::default()
        })
        .build()
        .fetch()
        .await
    {
        Ok(res) => {
            let mut json = Json(res);
            // Sort by events starting first
            json.data.result.edges.sort_by(|a, b| {
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

            json.format_start_date();
            json.description_to_html();
            Ok(json)
        }
        Err(e) => {
            error!("Error: {}", e);
            Err(AppError(e))
        }
    }
}

/// Body for `/search` route
///
/// * `query`:
#[derive(Serialize, Deserialize)]
pub struct SearchRequestBody {
    /// Search query
    query: Option<String>,
    /// Start date of event
    start_date: Option<String>,
    /// End date of event
    end_date: Option<String>,
    /// Events to return per page
    per_page: Option<u32>,
}

/// Handler for `/search` route.
pub async fn search_handler(
    Json(body): Json<SearchRequestBody>,
) -> Result<Json<GQLResponse>, AppError> {
    let search_request = SearchRequest::builder()
        .operation_name(OperationName2::eventSearchWithSeries)
        .variables(Variables {
            query: Some(body.query.unwrap_or_default()),
            start_date_range: "2024-09-08T18:16:46-04:00[US/Eastern]".into(),
            ..Default::default()
        })
        .build();
    debug!("Search request: {:#?}", search_request);
    // let search_request = SearchRequest::builder()
    //     .operation_name(OperationName2::eventSearchWithSeries)
    //     .variables(Variables {
    //         query: Some(body.query),
    //         start_date_range: body.start_date.unwrap_or(eod()),
    //         end_date_range: Some(body.end_date.unwrap_or(now())),
    //         first: body.per_page.unwrap_or(40) as i32,
    //         ..Default::default()
    //     })
    //     .build();

    info!("Fetching events");
    let response = match search_request.fetch().await {
        Ok(res) => res,
        Err(err) => {
            error!("Error: {}", err);
            return Err(AppError(err));
        }
    };
    info!("Events fetched");
    dbg!(&response);
    return Ok(Json(response));
}

// #[cfg(test)]
// mod tests {
//     use crate::routes::meetup::Response;
//     use crate::{
//         meetup::request::get_your_events_suggested_events::GetYourEventsSuggestedEventsResponse,
//         routes::{
//             app,
//             meetup::{Response, SearchRequestBody},
//         },
//     };
//     use axum::{
//         body::Body,
//         http::{self, Request},
//     };
//     use tower::{Service, ServiceExt}; // for `oneshot` and `ready`

//     #[tokio::test]
//     async fn meetup_search_status_code() {
//         let app = app();

//         let body = SearchRequestBody {
//             query: "programming".to_string(),
//             per_page: 10,
//             after: None,
//         };
//         let json_data = serde_json::to_string(&body).unwrap();

//         let response = app
//             .oneshot(
//                 Request::builder()
//                     .method(http::Method::POST)
//                     .uri("/meetup/search")
//                     .header("Content-Type", "application/json")
//                     .body(Body::from(json_data))
//                     .unwrap(),
//             )
//             .await
//             .unwrap();

//         assert_eq!(response.status(), http::StatusCode::OK);
//         let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
//         let body: Response = serde_json::from_slice(&body).unwrap();
//         assert_eq!(body.nodes.len(), 10 as usize);
//     }

//     /// test all meetup search result titles does not contain `online`
//     #[tokio::test]
//     async fn no_online_in_search_result_title() {
//         let app = app();

//         let body = SearchRequestBody {
//             query: "dating".to_string(),
//             per_page: 20,
//             after: None,
//         };
//         let json_data = serde_json::to_string(&body).unwrap();

//         let response = app
//             .oneshot(
//                 Request::builder()
//                     .method(http::Method::POST)
//                     .uri("/meetup/search")
//                     .header("Content-Type", "application/json")
//                     .body(Body::from(json_data))
//                     .unwrap(),
//             )
//             .await
//             .unwrap();

//         assert_eq!(response.status(), http::StatusCode::OK);
//         let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
//         let body: Response = serde_json::from_slice(&body).unwrap();
//         body.nodes.iter().for_each(|e| {
//             assert_eq!(e.title.contains("online"), false);
//         });
//     }

//     /// Make sure suggested events route returns events
//     #[tokio::test]
//     async fn can_get_suggested_events() {
//         let app = app();

//         let response = app
//             .oneshot(
//                 Request::builder()
//                     .method(http::Method::GET)
//                     .uri("/meetup/suggested")
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap();

//         assert_eq!(response.status(), http::StatusCode::OK);
//         let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
//         let body: GetYourEventsSuggestedEventsResponse = serde_json::from_slice(&body).unwrap();

//         assert_ne!(body.data.ranked_events.count, 0);
//         assert_ne!(body.data.ranked_events.edges.len(), 0);
//     }

//     /// test `/meetup/today` route returns successful status code
//     #[tokio::test]
//     async fn can_get_meetups_today() {
//         let app = app();
//         let response = app
//             .oneshot(
//                 Request::builder()
//                     .method(http::Method::GET)
//                     .uri("/meetup/today")
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap();
//         assert_eq!(response.status(), http::StatusCode::OK);
//         let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
//         let body: Response = serde_json::from_slice(&body).unwrap();
//         assert_ne!(body.data.ranked_events.count, 0);
//     }

//     /// test `/meetup/today` route with an after cursor returns different results
//     #[tokio::test]
//     async fn today_events_with_after_query_param() {
//         let mut app = app();

//         let request = Request::builder()
//             .uri("/meetup/today")
//             .body(Body::empty())
//             .unwrap();
//         let response = ServiceExt::<Request<Body>>::ready(&mut app)
//             .await
//             .unwrap()
//             .call(request)
//             .await
//             .unwrap();

//         assert_eq!(response.status(), http::StatusCode::OK);
//         let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
//         let first_request_body: Response = serde_json::from_slice(&body).unwrap();
//         assert_ne!(first_request_body.data.ranked_events.count, 0);

//         let request = Request::builder()
//             .uri(format!(
//                 "/meetup/today?after={}",
//                 &first_request_body.data.ranked_events.page_info.end_cursor
//             ))
//             .body(Body::empty())
//             .unwrap();

//         let response = ServiceExt::<Request<Body>>::ready(&mut app)
//             .await
//             .unwrap()
//             .call(request)
//             .await
//             .unwrap();

//         let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
//         let second_request_body: Response = serde_json::from_slice(&body).unwrap();

//         assert_ne!(
//             first_request_body.data.ranked_events.edges,
//             second_request_body.data.ranked_events.edges
//         );
//     }
// }
