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
async fn search(query: &str, page: i32, per_page: Option<i32>) -> String {
    let cache_key = format!("{}-{}-{}", query, page, per_page.unwrap());

    // let meetup: Search = meetup::search::Search::default();
    let meetup: Search = meetup::search::Search::default();
    let cache_value = CACHE.get(&cache_key.to_string()).await;
    let mut result: Search = Search::default();

    // if cache value does not exist
    if cache_value.is_none()
        // if length of nodes does not fit inside a single page
        || cache_value.as_ref().unwrap().value().data.results.edges.len() < per_page.unwrap() as usize
    {
        println!("making request");
        let mut cursor: Option<String> = None;
        let mut edge_vec: Vec<Edge> = vec![];
        loop {
            println!("cursor = {:?}", cursor);
            let search_result = meetup
                .search(
                    query.to_string(),
                    Some(meetup::search::EventType::physical),
                    cursor.clone(),
                    Some(100),
                )
                .await
                .unwrap();

            edge_vec.extend(search_result.data.results.edges.clone());

            // stop when we've fetched enough results
            if edge_vec.len() >= per_page.unwrap() as usize
                || search_result.data.results.pageInfo.endCursor == Some("".to_string())
            {
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
        let end = per_page.unwrap() * page;
        // if end is larger than the max size of vector, return vector max size
        if end > num_results as i32 {
            num_results as i32
        } else {
            end
        }
    };
    let vec_begin = vec_end - per_page.unwrap();

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
