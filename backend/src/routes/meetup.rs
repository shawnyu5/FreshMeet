use crate::meetup;
use crate::meetup::search_request::{RequestBody, RequestBuilder};
use lazy_static::lazy_static;
use retainer::Cache;
use rocket::response::status::BadRequest;
use rocket::serde::{json::Json, Serialize};
use serde::Deserialize;
use std::sync::{Arc, RwLock};
use types::meetup::search_request::{Edge, RsvpState};
use types::meetup::search_request::{Result_, SearchResult};
use types::meetup::search_response::Response;

lazy_static! {
    static ref CACHE: RwLock<Arc<Cache<String, SearchResult>>> =
        RwLock::new(Arc::new(Cache::<String, SearchResult>::new()));
    // static ref CURSOR: Arc<String> = Arc::new("".to_string());
    // static ref CURSOR: RwLock<Arc<String>> = RwLock::new(Arc::new("".to_string()));
}

// #[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
// /// response object for /search
// pub struct Response {
// page_info: PageInfo,
// nodes: Vec<Result_>,
// }

#[deprecated(since = "0.1.0", note = "use the post route /search instead")]
#[get("/search?<query>&<page>&<per_page>")]
/// search for a query on meetup
///
/// * `query`: the search query
/// * `page`: the page number
/// * `per_page`: number of nodes to return in a single page
pub async fn search(
    query: String,
    page: i32,
    per_page: i32,
) -> Result<Json<Response>, BadRequest<String>> {
    // make sure page is not less than 1
    if page < 1 {
        return Err(BadRequest(Some(
            "page number cannot be less than 1".to_string(),
        )));
    }

    let mut cursor = "".to_string();

    let mut result: SearchResult = SearchResult::default();

    loop {
        let request = meetup::search_request::RequestBuilder::new()
            .query(query.as_str())
            .first(20)
            .after(cursor)
            .build();

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
        result.data.results.edges.append(&mut filtered_vec);
        result.data.results.pageInfo = search_result.data.results.pageInfo.clone();
        cursor = search_result
            .data
            .results
            .pageInfo
            .endCursor
            .unwrap_or("".to_string());

        // if no next page, then stop
        if result.data.results.pageInfo.hasNextPage == false {
            break;
        }
    }

    if result.data.results.edges.len() == 0 {
        return Err(BadRequest(Some("no results found".to_string())));
    }

    let mut nodes: Vec<Result_> = vec![];

    for e in result.data.results.edges {
        nodes.push(e.node.result);
    }

    let vec_end: usize = {
        // calculate where the end of the page is
        // page = 2
        // per_page = 10
        let end = per_page * page; // end = 20

        // if end is larger than the max size of vector, return vector max size
        if end > nodes.len() as i32 {
            nodes.len()
        } else {
            end as usize
        }
    };
    let vec_begin: usize = {
        let result = vec_end as i32 - per_page;
        if result < 0 {
            0
        } else {
            result as usize
        }
    };

    // if we are not at the last page, show there are more pages
    if vec_end < nodes.len() {
        result.data.results.pageInfo.hasNextPage = true;
    } else {
        result.data.results.pageInfo.hasNextPage = false;
    }

    return Ok(Json(Response {
        page_info: result.data.results.pageInfo,
        nodes: nodes[vec_begin..vec_end].to_vec(),
    }));
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
/// data body for /search
///
/// * `query`: search query
/// * `per_page`: number of nodes to return in a single page
/// * `after`: pagination cursor
pub struct SearchData<'a> {
    pub query: &'a str,
    pub per_page: &'a str,
    pub after: Option<&'a str>,
}

#[post("/search", data = "<data>")]
pub async fn search_post(data: Json<SearchData<'_>>) -> Result<Json<Response>, BadRequest<String>> {
    struct SearchData<'a> {
        pub query: &'a str,
        pub per_page: i32,
        pub after: &'a str,
    }

    let data = SearchData {
        query: data.query,
        per_page: data.per_page.parse().unwrap_or(20),
        after: data.after.unwrap_or(""),
    };

    let request = RequestBuilder::new()
        .query(data.query)
        .first(data.per_page)
        .after(data.after.to_string())
        .build();

    let search_result = request.search().await.unwrap();
    let search_results: Vec<Result_> = search_result
        .events()
        .iter()
        .filter(|s| {
            !s.isAttending || s.rsvpState != types::meetup::search_request::RsvpState::CLOSED
        })
        .cloned()
        .collect();

    return Ok(Json(Response {
        page_info: search_result.data.results.pageInfo,
        nodes: search_results,
    }));
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rocket;
    use rocket::http::Status;

    #[rocket::async_test]
    /// test if the search endpoint returns a 200 response code
    async fn test_post_search() {
        use rocket::local::asynchronous::Client;
        let client = Client::tracked(rocket()).await.unwrap();
        let data = SearchData {
            query: "tech",
            per_page: "10",
            after: None,
        };
        let response = client
            .post(uri!("/meetup", search_post()))
            .body(serde_json::to_string(&data).unwrap())
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);
    }

    /// make sure pagination in the API works
    /// different pages should return different results
    #[rocket::async_test]
    async fn test_post_search_pagination() {
        use rocket::local::asynchronous::Client;

        let client = Client::tracked(rocket()).await.unwrap();
        let page_1_response = client
            .post(uri!("/meetup", search_post()))
            .body(
                serde_json::to_string(&SearchData {
                    query: "tech",
                    per_page: "10",
                    after: None,
                })
                .unwrap(),
            )
            .dispatch()
            .await;

        let page_1_response: Response = page_1_response.into_json().await.unwrap();
        let page_1_cursor = page_1_response
            .clone()
            .page_info
            .endCursor
            .unwrap_or("".to_string());

        let page_2_response = client
            .post(uri!("/meetup", search_post()))
            .body(
                serde_json::to_string(&SearchData {
                    query: "tech",
                    per_page: "10",
                    after: Some(page_1_cursor.as_str()),
                })
                .unwrap(),
            )
            .dispatch()
            .await;

        let page_2_response: Response = page_2_response.into_json().await.unwrap();

        // make sure both pages are different
        assert_ne!(page_1_response, page_2_response);
    }

    /// test all events have RSVP open
    #[rocket::async_test]
    async fn test_all_rsvp_open() {
        use rocket::local::asynchronous::Client;

        let client = Client::tracked(rocket()).await.unwrap();
        let page_1_response = client
            .post(uri!("/meetup", search_post()))
            .body(
                serde_json::to_string(&SearchData {
                    query: "tech",
                    per_page: "10",
                    after: None,
                })
                .unwrap(),
            )
            .dispatch()
            .await;

        let page_1_response: Response = page_1_response.into_json().await.unwrap();

        page_1_response.nodes.iter().for_each(|n| {
            assert_eq!(n.isAttending, false);
            assert_ne!(n.rsvpState, RsvpState::CLOSED);
        });
    }
}
