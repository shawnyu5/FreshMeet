mod meetup;
mod traits;
use crate::meetup::search::{Edge, PageInfo, RequestBody, Result_, RsvpState, SearchResult};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
/// data body for /search
///
/// * `query`: search query
/// * `page`: page number
/// * `per_page`: number of nodes to return in a single page
pub struct SearchData<'a> {
    pub query: &'a str,
    pub page: i32,
    pub per_page: i32,
    pub start_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
/// response object for /search
///
/// * `page_info`: meta data for current page
/// * `nodes`: list of event nodes
pub struct Response {
    pub page_info: PageInfo,
    pub nodes: Vec<Result_>,
}

impl IntoIterator for Response {
    type Item = Result_;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        return self.nodes.into_iter();
    }
}

pub async fn search(data: SearchData<'_>) -> Result<Response, String> {
    // make sure page is not less than 1
    if data.page < 1 {
        return Err("page number cannot be less than 1".to_string());
    }

    // let cache_read = CACHE.read().unwrap();
    // let cache_key = "search".to_string();
    // let cache_value = cache_read.get(&cache_key).await;
    let mut cursor = "".to_string();

    let mut result: SearchResult = SearchResult::default();

    // if cache value does not exist
    // if cache_value.is_none() {
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
        return Err("no results found".to_string());
    }

    let mut nodes: Vec<Result_> = vec![];

    for e in result.data.results.edges {
        nodes.push(e.node.result);
    }

    let vec_end: usize = {
        // calculate where the end of the page is
        // page = 2
        // per_page = 10
        let end = data.per_page * data.page; // end = 20

        // if end is larger than the max size of vector, return vector max size
        if end > nodes.len() as i32 {
            nodes.len()
        } else {
            end as usize
        }
    };
    let vec_begin: usize = {
        let result = vec_end as i32 - data.per_page;
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

    return Ok(Response {
        page_info: result.data.results.pageInfo,
        nodes: nodes[vec_begin..vec_end].to_vec(),
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    /// test if the search endpoint returns a 200 response code
    async fn test_search() {
        let data = SearchData {
            query: "tech",
            page: 1,
            per_page: 10,
            start_date: None,
        };
        let result = search(data).await;
        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    /// make sure pagination in the API works
    /// different pages should return different results
    async fn test_post_search_pagination() {
        let page_1_response = search(SearchData {
            query: "tech",
            page: 1,
            per_page: 10,
            start_date: None,
        })
        .await;

        let page_2_response = search(SearchData {
            query: "tech",
            page: 2,
            per_page: 10,
            start_date: None,
        })
        .await;

        assert_eq!(&page_1_response.is_ok(), &true);
        assert_eq!(&page_2_response.is_ok(), &true);
        // make sure both pages are different
        assert_ne!(page_1_response, page_2_response);
    }

    #[tokio::test]
    /// test making a request with a page number less than 1 will return a status code 400 bad request
    async fn test_post_invalid_page_number() {
        let response = search(SearchData {
            query: "tech",
            page: 0,
            per_page: 10,
            start_date: None,
        })
        .await;

        assert_eq!(&response.is_err(), &true);
        assert_eq!(
            response.err().unwrap(),
            "page number cannot be less than 1".to_string()
        );
    }
}
