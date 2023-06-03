use serde::{Deserialize, Serialize};

use super::search_request::{PageInfo, Result_};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
/// response object for /search
pub struct Response {
    pub page_info: PageInfo,
    pub nodes: Vec<Result_>,
}

impl IntoIterator for Response {
    type Item = Result_;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}
