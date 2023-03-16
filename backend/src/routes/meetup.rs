use std::{sync::Arc, time::Duration};

use crate::meetup::search::{Edge, EventType, RsvpState, Search};
use lazy_static::lazy_static;
use retainer::Cache;
use rocket::response::status::BadRequest;

lazy_static! {
    pub static ref CACHE: Arc<Cache<String, Search>> = Arc::new(Cache::<String, Search>::new());
}

#[get("/search?<query>&<page>&<per_page>")]
/// search for a query on meetup
///
/// * `query`: the search query
/// * `page`: the page number
/// * `per_page`: number of nodes to return in a single page
pub async fn search(query: &str, page: i32, per_page: i32) -> Result<String, BadRequest<String>> {
    let cache_key = format!("{}-{}-{}", query, page, per_page);

    let meetup: Search = Search::default();
    let cache_value = CACHE.get(&cache_key.to_string()).await;
    let mut result: Search = Search::default();
    let mut cursor: Option<String> = None;

    // make sure page is not less than 1
    if page < 1 {
        return Err(BadRequest(Some(
            "page number cannot be less than 1".to_string(),
        )));
    }

    // if cache value does not exist
    if cache_value.is_none()
    // if length of nodes does not fit inside a single page
    || cache_value.as_ref().unwrap().value().data.results.edges.len() < per_page as usize
    {
        println!("making request");
        // keep track of the cursor of the previous search
        // contains all search results
        let mut edge_vec: Vec<Edge> = vec![];

        loop {
            println!("cursor = {:?}", cursor);
            let search_result = meetup
                .search(
                    query.to_string(),
                    EventType::physical,
                    cursor.clone(),
                    // fetch 10 pages of results
                    page * 10,
                )
                .await
                .unwrap();

            let mut filtered_vec: Vec<Edge> = vec![];
            // filter out events that I am attending
            for edge in search_result.data.results.edges {
                if edge.node.result.isAttending.unwrap_or(false)
                    || edge.node.result.rsvpState == RsvpState::CLOSED
                {
                    continue;
                }
                filtered_vec.push(edge);
            }
            edge_vec.extend(filtered_vec.clone());

            // stop when we've fetched enough results. Or if there no more nodes to fetch
            if edge_vec.len() >= per_page as usize
                || search_result.data.results.pageInfo.endCursor == Some("".to_string())
            {
                // add response from meetup api to results
                result.data.results.edges.append(&mut edge_vec);
                break;
            }
            cursor = search_result.data.results.pageInfo.endCursor.clone();
        }
    } else {
        result = cache_value.unwrap().value().clone();
    }
    // number of nodes in the search result
    let num_results = result.data.results.edges.len();

    // cache the entire search result
    CACHE
        .insert(
            cache_key.to_string(),
            result.clone(),
            Duration::from_secs(20 * 60),
        )
        .await;
    let nodes = &result.data.results.edges;
    let vec_end = {
        // calculate where the end of the page is
        let end = per_page * page;
        // if end is larger than the max size of vector, return vector max size
        if end > num_results as i32 {
            num_results as i32
        } else {
            end
        }
    };
    let vec_begin = vec_end - per_page;

    // println!("vec_begin = {}", vec_begin);
    // println!("vec_end = {}", vec_end);
    // println!("num_results = {}", num_results);
    return Ok(serde_json::to_string_pretty(&nodes[vec_begin as usize..vec_end as usize]).unwrap());
}

#[cfg(test)]
mod test {
    // use super::rocket;
    use super::*;
    use crate::rocket;
    use rocket::http::Status;

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

        let page_2_response = client
            .get(uri!("/meetup", search("tech", 2, 10)))
            .dispatch()
            .await;

        assert_eq!(page_1_response.status(), Status::Ok);
        assert_eq!(page_2_response.status(), Status::Ok);

        // make sure both pages are different
        assert_ne!(
            page_1_response.into_string().await.unwrap(),
            page_2_response.into_string().await.unwrap()
        );
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
}
