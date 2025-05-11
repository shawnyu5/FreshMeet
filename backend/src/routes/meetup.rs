//! Route handlers for meetups

use crate::meetup::query::common::OperationName2;
use crate::meetup::query::request::gql2::{GQLResponse, SearchRequest, Variables};
use crate::meetup::response::{Event, PageInfo};
use crate::utils::now;
use axum::{extract::Query, Json};
use chrono::{DateTime, Timelike, Utc};
use chrono_tz::America::New_York;
use common_axum::axum::AppError;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};
use utoipa::{IntoParams, ToSchema};

/// response body for meetup search
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub page_info: PageInfo,
    pub nodes: Vec<Event>,
}

/// Query parameters for `/today` route
#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct RecommendedMeetupsQueryParams {
    /// Start date of event
    pub start_date: DateTime<Utc>,
    /// End date of event
    pub end_date: DateTime<Utc>,
    /// After cursor
    pub after: Option<String>,
}

/// Gets recommended meetups
#[utoipa::path(
    get,
    path = "/recommended",
    responses(
        (status = 200, description = "Found recommended meetups successfully", body = GQLResponse),
        (status = 500, description = "Failed to fetch meetups", body = String)
    ),
    params(
        RecommendedMeetupsQueryParams
    )
)]
pub async fn recommended_meetups_handler(
    query: Query<RecommendedMeetupsQueryParams>,
) -> Result<Json<GQLResponse>, AppError> {
    let start_date_range = query
        .start_date
        .with_timezone(&New_York)
        .with_hour(0)
        .expect("Failed to set start time to beginning of day")
        .to_rfc3339();

    let end_date_range = query
        .end_date
        .with_timezone(&New_York)
        .with_hour(23)
        .expect("Failed to set end time to end of day")
        .to_rfc3339();

    match SearchRequest::builder()
        .operation_name(OperationName2::recommendedEventsWithSeries)
        .variables(Variables {
            first: 200,
            start_date_range,
            end_date_range: Some(end_date_range),
            after: query.after.clone(),
            ..Default::default()
        })
        .build()
        .fetch()
        .await
    {
        Ok(mut res) => {
            // Sort by events starting first
            debug_assert!(
                res.data.is_some(),
                "There should always be data here. Something is wrong if there is no data"
            );
            res.sort();
            res.generate_google_maps_url();
            res.format();

            if let Some(data) = &res.data {
                debug!("Number of events: {}", data.result.total_count);
                debug!("After cursor: {:?}", data.result.page_info.end_cursor);
            }

            Ok(Json(res))
        }
        Err(e) => {
            error!("Error: {}", e);
            Err(AppError(e))
        }
    }
}

/// Body for `/search` route
#[derive(Serialize, Deserialize, ToSchema)]
pub struct SearchRequestBody {
    /// Search query
    query: Option<String>,
    /// Start date of event
    start_date: Option<String>,
    // /// End date of event
    // end_date: Option<String>,
    /// Events to return per page
    per_page: Option<u32>,
}

/// Searches meetups. Event end date will not be set, only even start date will be taken into account.
#[utoipa::path(
    post,
    path = "/search",
    responses(
        (status = 200, description = "Successfully returned searched meetups", body = GQLResponse),
        (status = 500, description = "Failed to search for meetups", body = String)
    ),
    request_body = SearchRequestBody

)]
pub async fn search_handler(
    Json(body): Json<SearchRequestBody>,
) -> Result<Json<GQLResponse>, AppError> {
    let search_request = SearchRequest::builder()
        .operation_name(OperationName2::eventSearchWithSeries)
        .variables(Variables {
            query: Some(body.query.unwrap_or_default()),
            start_date_range: body.start_date.unwrap_or(now()),
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
        Ok(mut res) => {
            // Sort by events starting first
            debug_assert!(
                res.data.is_some(),
                "There should always be data here. Something is wrong if there is no data"
            );
            res.sort();
            res.format();
            res
        }
        Err(err) => {
            error!("Error: {}", err);
            return Err(AppError(err));
        }
    };
    info!("Events fetched");
    return Ok(Json(response));
}

// #[instrument(skip_all)]
// #[utoipa::path(
//     get,
//     path = "/rsvp",
//     responses(
//         (status = 200, description = "Successfully returned RSVP events", body = RsvpResponse),
//         (status = 500, description = "Failed to fetch RSVP events", body = String)
//     ),
//     request_body = SearchRequestBody

// )]
// pub async fn get_rsvp_events() -> Result<Json<GQLResponse>, app_error_v2::AppError> {
//     let request = RsvpEvents::builder().build();
//     let rsvp_events = request.rsvp().await?;
//     return Ok(Json());
// }

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
//
