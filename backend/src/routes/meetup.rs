use crate::meetup;
use crate::meetup::search_request::RequestBody;
use crate::meetup::search_types::*;
use lazy_static::lazy_static;
use retainer::Cache;
use rocket::response::status::BadRequest;
use rocket::serde::{json::Json, Serialize};
use serde::Deserialize;
use std::sync::{Arc, RwLock};

lazy_static! {
    static ref CACHE: RwLock<Arc<Cache<String, SearchResult>>> =
        RwLock::new(Arc::new(Cache::<String, SearchResult>::new()));
    // static ref CURSOR: Arc<String> = Arc::new("".to_string());
    // static ref CURSOR: RwLock<Arc<String>> = RwLock::new(Arc::new("".to_string()));
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
/// response object for /search
///
/// * `page_info`: meta data for current page
/// * `nodes`: list of event nodes
pub struct Response {
    page_info: PageInfo,
    nodes: Vec<Result_>,
}

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
        let mut request = meetup::search_request::RequestBuilder::new()
            .query(query.as_str())
            .first(20)
            .build();
        request.variables.after = cursor.to_string().clone();

        // let mut request = meetup::search_result::default();
        // request.variables.query = query.to_string();
        // request.variables.first = 20;

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
/// * `page`: page number
/// * `per_page`: number of nodes to return in a single page
pub struct SearchData<'a> {
    pub query: &'a str,
    pub page: &'a str,
    pub per_page: &'a str,
    pub start_date: Option<String>,
}

#[post("/search", data = "<data>")]
pub async fn search_post(data: Json<SearchData<'_>>) -> Result<Json<Response>, BadRequest<String>> {
    struct SearchData<'a> {
        pub query: &'a str,
        pub page: i32,
        pub per_page: i32,
        pub start_date: Option<String>,
    }
    let data = SearchData {
        query: data.query,
        page: data.page.parse().unwrap_or(-1),
        per_page: data.per_page.parse().unwrap_or(-1),
        start_date: data.start_date.clone(),
    };
    // make sure page is not less than 1
    if data.page < 1 {
        return Err(BadRequest(Some(
            "page number cannot be less than 1".to_string(),
        )));
    }

    let mut cursor = "".to_string();

    let mut result: SearchResult = SearchResult::default();

    // if cache value does not exist
    loop {
        let mut request = RequestBody::default();
        request.variables.query = data.query.to_string();
        request.variables.first = 20;
        request.variables.after = cursor.to_string().clone();
        request.variables.startDate = data.start_date.clone();

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
    // } else {
    // result = cache_value.unwrap().value().clone();
    // }

    // create new cache with updated search results
    // let updated_cache = Cache::<String, SearchResult>::new();
    // updated_cache
    // .insert(cache_key, result.clone(), Duration::from_secs(20 * 60))
    // .await;

    // update global cache
    // let mut cache_write = CACHE.write().unwrap();
    // *cache_write = Arc::new(updated_cache);

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
        let end: i32 = data.per_page * data.page; // end = 20

        // if end is larger than the max size of vector, return vector max size
        if end > nodes.len() as i32 {
            nodes.len()
        } else {
            end as usize
        }
    };
    let vec_begin: usize = {
        let result: i32 = vec_end as i32 - data.per_page;
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::rocket;
    use rocket::http::Status;

    #[rocket::async_test]
    /// test if the search endpoint returns a 200 response code
    async fn test_search() {
        use rocket::local::asynchronous::Client;
        let client = Client::tracked(rocket()).await.unwrap();
        let response = client
            .get(uri!("/meetup", search("tech", 1, 10)))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);
    }

    #[rocket::async_test]
    /// test if the search endpoint returns a 200 response code
    async fn test_post_search() {
        use rocket::local::asynchronous::Client;
        let client = Client::tracked(rocket()).await.unwrap();
        let data = SearchData {
            query: "tech",
            page: "1",
            per_page: "10",
            start_date: None,
        };
        let response = client
            .post(uri!("/meetup", search_post()))
            .body(serde_json::to_string(&data).unwrap())
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
            .get(uri!("/meetup", search("tech", 1, 10)))
            .dispatch()
            .await;

        // assert_eq!(page_1_response.status(), Status::Ok);

        let page_1_response: Response = page_1_response.into_json().await.unwrap();
        let page_2_response = client
            .get(uri!("/meetup", search("tech", 2, 10)))
            .dispatch()
            .await;
        // assert_eq!(page_2_response.status(), Status::Ok);

        let page_2_response: Response = page_2_response.into_json().await.unwrap();

        // make sure both pages are different
        assert_ne!(page_1_response, page_2_response);
    }

    #[rocket::async_test]
    /// make sure pagination in the API works
    /// different pages should return different results
    async fn test_post_search_pagination() {
        use rocket::local::asynchronous::Client;

        // let data = SearchData {
        // query: "tech",
        // page: 1,
        // per_page: 10,
        // };
        // let response = client
        // .post(uri!("/meetup", search_post()))
        // .body(serde_json::to_string(&data).unwrap())
        // .dispatch()
        // .await;

        let client = Client::tracked(rocket()).await.unwrap();
        let page_1_response = client
            .post(uri!("/meetup", search_post()))
            .body(
                serde_json::to_string(&SearchData {
                    query: "tech",
                    page: "1",
                    per_page: "10",
                    start_date: None,
                })
                .unwrap(),
            )
            .dispatch()
            .await;

        let page_1_response: Response = page_1_response.into_json().await.unwrap();
        let page_2_response = client
            .post(uri!("/meetup", search_post()))
            .body(
                serde_json::to_string(&SearchData {
                    query: "tech",
                    page: "2",
                    per_page: "10",
                    start_date: None,
                })
                .unwrap(),
            )
            .dispatch()
            .await;

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
            .get(uri!("/meetup", search("tech", 0, 10)))
            .dispatch()
            .await;
        assert_eq!(res.status(), Status::BadRequest);
        assert_eq!(
            res.into_string().await.unwrap(),
            "page number cannot be less than 1".to_string()
        );
    }

    #[rocket::async_test]
    /// test making a request with a page number less than 1 will return a status code 400 bad request
    async fn test_post_invalid_page_number() {
        use rocket::local::asynchronous::Client;
        let client = Client::tracked(rocket()).await.unwrap();
        let res = client
            .post(uri!("/meetup", search_post()))
            .body(
                serde_json::to_string(&SearchData {
                    query: "tech",
                    page: "0",
                    per_page: "10",
                    start_date: None,
                })
                .unwrap(),
            )
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
            .get(uri!("/meetup", search("tech", 1, 4)))
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::Ok);
        let res = res.into_json::<Response>().await.unwrap();
        assert_ne!(res.page_info.endCursor, None);
        // there should be more pages
        assert_eq!(res.page_info.hasNextPage, true);

        let res = client
            .get(uri!("/meetup", search("tech", 200, 4)))
            .dispatch()
            .await;
        assert_eq!(res.status(), Status::Ok);
        let res = res.into_json::<Response>().await.unwrap();
        assert_eq!(res.page_info.hasNextPage, false);
    }

    #[rocket::async_test]
    /// make sure there is a valid page info
    async fn test_post_page_info() {
        use rocket::local::asynchronous::Client;
        let client = Client::tracked(rocket()).await.unwrap();
        let res = client
            .post(uri!("/meetup", search_post()))
            .body(
                serde_json::to_string(&SearchData {
                    query: "tech",
                    page: "1",
                    per_page: "4",
                    start_date: None,
                })
                .unwrap(),
            )
            .dispatch()
            .await;

        assert_eq!(res.status(), Status::Ok);
        let res = res.into_json::<Response>().await.unwrap();
        assert_ne!(res.page_info.endCursor, None);
        // there should be more pages
        assert_eq!(res.page_info.hasNextPage, true);

        let res = client
            .post(uri!("/meetup", search_post()))
            .body(
                serde_json::to_string(&SearchData {
                    query: "tech",
                    page: "200",
                    per_page: "4",
                    start_date: None,
                })
                .unwrap(),
            )
            .dispatch()
            .await;
        assert_eq!(res.status(), Status::Ok);
        let res = res.into_json::<Response>().await.unwrap();
        assert_eq!(res.page_info.hasNextPage, false);
    }
}
