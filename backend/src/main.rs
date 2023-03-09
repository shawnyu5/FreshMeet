#[macro_use]
extern crate rocket;

use lazy_static::lazy_static;
use meetup::search::Search;
use retainer::*;
use std::sync::Arc;
use std::time::Duration;

use crate::meetup::search::Edge;
use crate::meetup::search::Node;
mod eventbrite;
mod meetup;

lazy_static! {
    static ref CACHE: Arc<Cache<String, Search>> = Arc::new(Cache::<String, Search>::new());
}

#[get("/search?<query>&<page>&<per_page>")]
/// search for a query on meetup
///
/// * `query`: the search query
/// * `page`: the page number
/// * `per_page`: number of nodes to return in a single page
async fn search(query: &str, page: i32, per_page: i32) -> String {
    let cache_key = format!("{}-{}-{}", query, page, per_page);

    // let meetup: Search = meetup::search::Search::default();
    let meetup: Search = meetup::search::Search::default();
    let cache_value = CACHE.get(&cache_key.to_string()).await;
    let mut result: Search = Search::default();
    let mut cursor: Option<String> = None;

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
                    meetup::search::EventType::physical,
                    cursor.clone(),
                    100,
                )
                .await
                .unwrap();

            edge_vec.extend(search_result.data.results.edges.clone());

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
    // let nodes = &result.data.results.edges[vec_start as usize..vec_end as usize];
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

    println!("vec_begin = {}", vec_begin);
    println!("vec_end = {}", vec_end);
    println!("num_results = {}", num_results);
    return serde_json::to_string_pretty(&nodes[vec_begin as usize..vec_end as usize]).unwrap();
}

#[get("/")]
fn index() -> &'static str {
    return "Hello";
}

#[launch]
fn rocket() -> _ {
    println!("Starting on port 8000");
    let cache_clone = CACHE.clone();

    // don't forget to monitor your cache to evict entries
    // let monitor =
    tokio::spawn(async move { cache_clone.monitor(4, 0.25, Duration::from_secs(3)).await });

    rocket::build()
        .mount("/", routes![index])
        .mount("/meetup", routes![search])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use super::*;
    use rocket::http::Status;

    #[rocket::async_test]
    /// make sure pagination in the API works
    /// different pages will return different results
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
            &page_1_response.into_string().await.unwrap(),
            &page_2_response.into_string().await.unwrap()
        );
    }
}
