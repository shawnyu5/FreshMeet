use crate::meetup::search::Edge;
use crate::routes::meetup::search;
use rand::seq::SliceRandom;
use rocket::response::status::BadRequest;
// use lazy_static::lazy_static;

// use retainer::Cache;

// lazy_static! {
// // TODO: should prob implement caching
// // pub static ref CACHE: Arc<Cache<String, Search>> = Arc::new(Cache::<String, Search>::new());
// }

#[get("/tech-events?<page>&<per_page>")]
pub async fn tech_events(page: i32, per_page: i32) -> Result<String, BadRequest<String>> {
    if page < 1 {
        return Err(BadRequest(Some(
            "page number cannot be less than 1".to_string(),
        )));
    }
    let meetup_queries = vec!["coding", "tech", "programming"];
    let mut meetups: Vec<Edge> = vec![];

    for query in meetup_queries {
        let search_results = search(query, page, per_page).await.unwrap();
        let search_results: Vec<Edge> = serde_json::from_str(&search_results).unwrap();
        meetups.extend(search_results.clone());
    }
    meetups.shuffle(&mut rand::thread_rng());

    let num_results = meetups.len();

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
    return Ok(
        serde_json::to_string_pretty(&meetups[vec_begin as usize..vec_end as usize]).unwrap(),
    );
}
#[cfg(test)]
mod test {
    // use super::rocket;
    use super::*;
    use crate::rocket;
    use rocket::http::Status;

    #[rocket::async_test]
    /// should return a 200 status code
    async fn test_it_works() {
        use rocket::local::asynchronous::Client;
        let client = Client::tracked(rocket()).await.unwrap();
        let res = client.get(uri!(tech_events(1, 4))).dispatch().await;
        assert_eq!(res.status(), Status::Ok);
    }

    #[rocket::async_test]
    /// check we only get 4 nodes back
    async fn test_num_results() {
        use rocket::local::asynchronous::Client;
        let client = Client::tracked(rocket()).await.unwrap();
        let res = client.get(uri!(tech_events(1, 4))).dispatch().await;
        assert_eq!(
            serde_json::from_str::<Vec<Edge>>(&res.into_string().await.unwrap())
                .unwrap()
                .len(),
            4
        );
    }

    #[rocket::async_test]
    /// make sure pagination in the API works
    /// different pages should return different results
    async fn test_search_pagination() {
        use rocket::local::asynchronous::Client;

        let client = Client::tracked(rocket()).await.unwrap();
        let page_1_response = client.get(uri!(tech_events(1, 10))).dispatch().await;

        let page_2_response = client.get(uri!(tech_events(2, 10))).dispatch().await;

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
        let res = client.get(uri!(tech_events(0, 10))).dispatch().await;
        assert_eq!(res.status(), Status::BadRequest);
        assert_eq!(
            res.into_string().await.unwrap(),
            "page number cannot be less than 1".to_string()
        );
    }
}
