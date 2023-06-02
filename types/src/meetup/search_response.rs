use serde::{Deserialize, Serialize};

use super::search_request::{PageInfo, Result_};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
/// response object for /search
pub struct Response {
    page_info: PageInfo,
    nodes: Vec<Result_>,
}
