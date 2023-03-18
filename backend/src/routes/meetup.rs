use std::{sync::Arc, time::Duration};

use crate::meetup::search::{request_body, Edge, PageInfo, RsvpState, Search, SearchResult};
use lazy_static::lazy_static;
use retainer::Cache;
use rocket::response::status::BadRequest;
use rocket::serde::{json::Json, Serialize};
use serde::Deserialize;

lazy_static! {
    pub static ref CACHE: Arc<Cache<String, Search>> = Arc::new(Cache::<String, Search>::new());
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
/// response object for /search
///
/// * `page_info`: meta data for current page
/// * `nodes`: list of event nodes
pub struct Response {
    page_info: PageInfo,
    nodes: Vec<SearchResult>,
}

#[get("/search?<query>&<page>&<per_page>&<after>")]
/// search for a query on meetup
///
/// * `query`: the search query
/// * `page`: the page number
/// * `per_page`: number of nodes to return in a single page
/// * `after`: cursor of the previous search
pub async fn search(
    query: &str,
    page: i32,
    per_page: i32,
    after: Option<String>,
) -> Result<Json<Response>, BadRequest<String>> {
    let cache_key = format!(
        "{}-{}-{}-{}",
        query,
        page,
        per_page,
        after.as_ref().unwrap_or(&"".to_string())
    );

    let mut request = request_body::Body::default();
    let cache_value = CACHE.get(&cache_key.to_string()).await;
    let mut result: Search = Search::default();

    // make sure page is not less than 1
    if page < 1 {
        return Err(BadRequest(Some(
            "page number cannot be less than 1".to_string(),
        )));
    }

    // if cache value does not exist
    if cache_value.is_none() {
        println!("No cache found");

        request.variables.query = query.to_string();
        request.variables.first = per_page;
        request.variables.after = after.clone().unwrap_or("".to_string());
        let search_result = request.search().await.unwrap();

        let mut filtered_vec: Vec<Edge> = vec![];
        for edge in search_result.data.results.edges {
            // TODO: request needs to be send with cookie information to get isAttending info
            // filter out events that I am attending, or if RSVP is closed
            if edge.node.result.isAttending || edge.node.result.rsvpState == RsvpState::CLOSED {
                continue;
            }
            filtered_vec.push(edge);
        }
        result.data.results.edges = filtered_vec;
        result.data.results.pageInfo = search_result.data.results.pageInfo;
    } else {
        result = cache_value.unwrap().value().clone();
    }

    if result.data.results.edges.len() == 0 {
        return Err(BadRequest(Some("no results found".to_string())));
    }

    // cache the entire search result
    CACHE
        .insert(
            cache_key.to_string(),
            result.clone(),
            Duration::from_secs(20 * 60),
        )
        .await;

    let mut nodes: Vec<SearchResult> = vec![];

    for e in result.data.results.edges {
        nodes.push(e.node.result);
    }

    return Ok(Json(Response {
        page_info: result.data.results.pageInfo,
        nodes,
    }));
}

#[cfg(test)]
mod test {
    // use super::rocket;
    use super::*;
    use crate::rocket;
    use rocket::http::Status;

    #[rocket::async_test]
    /// test if the search endpoint returns a 200 response code
    async fn test_search() {
        use rocket::local::asynchronous::Client;
        let client = Client::tracked(rocket()).await.unwrap();
        let response = client
            .get(uri!("/meetup", search("tech", 1, 10, None::<String>)))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);
    }

    #[rocket::async_test]
    /// make sure pagination in the API works
    /// different pages should return different results
    async fn test_search_pagination() {
        use rocket::local::asynchronous::Client;

        let client = Client::tracked(rocket()).await.unwrap();
        let page_1_response = client
            .get(uri!("/meetup", search("tech", 1, 10, None::<String>)))
            .dispatch()
            .await;

        assert_eq!(page_1_response.status(), Status::Ok);

        let page_1_response: Response = page_1_response.into_json().await.unwrap();
        let page_2_response = client
            .get(uri!(
                "/meetup",
                search("tech", 2, 10, page_1_response.page_info.endCursor.as_ref())
            ))
            .dispatch()
            .await;
        assert_eq!(page_2_response.status(), Status::Ok);

        let page_2_response: Response = page_2_response.into_json().await.unwrap();

        // make sure both pages are different
        assert_ne!(page_1_response, page_2_response);
    }
    #[rocket::async_test]
    /// test making a request with a page number less than 1 will return a status code 400 bad request
    async fn test_invalid_page_number() {
        use rocket::local::asynchronous::Client;
        let client = Client::tracked(rocket()).await.unwrap();
        let res = client
            .get(uri!("/meetup", search("tech", 0, 10, None::<String>)))
            .dispatch()
            .await;
        assert_eq!(res.status(), Status::BadRequest);
        assert_eq!(
            res.into_string().await.unwrap(),
            "page number cannot be less than 1".to_string()
        );
    }

    #[rocket::async_test]
    /// make sure there is a valid page info
    async fn test_page_info() {
        use rocket::local::asynchronous::Client;
        let client = Client::tracked(rocket()).await.unwrap();
        let res = client
            .get(uri!("/meetup", search("tech", 1, 10, None::<String>)))
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::Ok);
        let res = res.into_json::<Response>().await.unwrap();
        assert_ne!(res.page_info.endCursor, None);
    }
}
