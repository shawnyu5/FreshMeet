use std::marker::PhantomData;

use super::request::request::{EventKeywordSearchRequest, Variables};

/// Builder for building a meetup request
///
/// T: the type of request to build
#[derive(Debug)]
pub struct RequestBuilder<T> {
    operation_name: PhantomData<T>,
    query: Option<String>,
    first: i32,
    after: Option<String>,
}

impl<T> RequestBuilder<T>
where
    T: Default,
{
    /// construct a new request builder
    pub fn new() -> RequestBuilder<T>
    where
        T: Default,
    {
        return RequestBuilder {
            operation_name: PhantomData::<T>,
            query: None,
            first: 10,
            after: None,
        };
    }
}

impl RequestBuilder<EventKeywordSearchRequest> {
    /// set the query to search for
    pub fn query(&mut self, query: &str) -> &mut Self {
        self.query = Some(query.to_string());
        return self;
    }

    /// number of results to return
    pub fn per_page(&mut self, per_page: i32) -> &mut Self {
        self.first = per_page;
        return self;
    }

    /// set the after cursor
    pub fn after(&mut self, after: Option<String>) -> &mut Self {
        self.after = after;
        return self;
    }

    /// build the request body
    pub fn build(&mut self) -> EventKeywordSearchRequest {
        return EventKeywordSearchRequest {
            variables: Variables {
                query: self.query.clone(),
                first: self.first,
                after: self.after.clone(),
                ..Default::default()
            },
            ..EventKeywordSearchRequest::default()
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_construct() {
        let mut builder = RequestBuilder::<EventKeywordSearchRequest>::new();
        builder.query("tech");
        builder.per_page(10);
        let request = builder.build();

        assert_eq!(request.variables.query, Some("tech".to_string()));
    }
}
