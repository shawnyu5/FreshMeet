use serde::{Deserialize, Serialize};

use super::search_request::{PageInfo, Result_};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
/// response object for /search
pub struct Response {
    pub page_info: PageInfo,
    pub nodes: Vec<Result_>,
}
