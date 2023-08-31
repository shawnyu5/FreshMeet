use crate::meetup::request_builder::Builder;

use super::category_search::CategorySearchRequest;

#[derive(Debug, Default)]
pub struct CategorySearchRequestBuilder {}

impl Builder<CategorySearchRequest> for CategorySearchRequestBuilder {
    fn new() -> Self {
        return CategorySearchRequestBuilder::default();
    }

    fn build(&mut self) -> CategorySearchRequest {
        return CategorySearchRequest::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_create() {
        let builder = CategorySearchRequestBuilder::new();
        dbg!(builder);
    }

    #[test]
    fn can_build() {
        let request = CategorySearchRequestBuilder::new().build();
        assert_eq!(request.operation_name, "categorySearch");
    }
}
