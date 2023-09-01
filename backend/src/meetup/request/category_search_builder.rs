use crate::meetup::request_builder::Builder;

use super::category_search::{CategorySearchRequest, Variables};

#[derive(Debug, Default)]
pub struct CategorySearchRequestBuilder {
    /// The after cursor
    after: Option<String>,
    /// Number of results to return
    first: Option<i32>,
}

impl CategorySearchRequestBuilder {
    pub fn after(&mut self, after: Option<String>) -> &mut Self {
        self.after = after;
        return self;
    }
    pub fn per_page(&mut self, first: i32) -> &mut Self {
        self.first = Some(first);
        return self;
    }
}
impl Builder<CategorySearchRequest> for CategorySearchRequestBuilder {
    fn new() -> Self {
        return CategorySearchRequestBuilder::default();
    }

    fn build(&mut self) -> CategorySearchRequest {
        return CategorySearchRequest {
            variables: Variables {
                after: self.after.clone(),
                first: self.first.unwrap_or_else(|| 30),
                ..Default::default()
            },
            ..Default::default()
        };
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

    #[test]
    fn can_set_after() {
        let request = CategorySearchRequestBuilder::new()
            .after(Some("test".to_string()))
            .build();
        assert_eq!(request.variables.after, Some("test".to_string()));
    }

    #[test]
    fn can_set_first() {
        let request = CategorySearchRequestBuilder::new().per_page(22).build();
        assert_eq!(request.variables.first, 22);
    }
}
