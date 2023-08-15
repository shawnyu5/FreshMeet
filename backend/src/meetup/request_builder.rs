use super::request::{OperationName, RequestBody, Variables};

/// Builder for building a meetup request
#[derive(Debug)]
pub struct RequestBuilder {
    pub operation_name: OperationName,
    pub query: Option<String>,
    pub first: i32,
    pub after: Option<String>,
}

impl RequestBuilder {
    /// construct a new request builder
    pub fn new() -> RequestBuilder {
        return RequestBuilder {
            operation_name: OperationName::getYourEventsSuggestedEvents,
            query: None,
            first: 10,
            after: None,
        };
    }

    /// set the query to search for
    pub fn query(&mut self, query: &str) -> &mut RequestBuilder {
        self.query = Some(query.to_string());
        return self;
    }

    /// number of results to return
    pub fn per_page(&mut self, per_page: i32) -> &mut RequestBuilder {
        self.first = per_page;
        return self;
    }

    /// set the after cursor
    pub fn after(&mut self, after: Option<String>) -> &mut RequestBuilder {
        self.after = after;
        return self;
    }

    /// build the request body
    pub fn build(&mut self) -> RequestBody {
        // if a query is supplied, then its a search operation
        if self.query.is_some() {
            self.operation_name = OperationName::eventKeywordSearch;
        } else {
            self.operation_name = OperationName::getYourEventsSuggestedEvents;
        }
        return RequestBody {
            variables: Variables {
                query: self.query.clone(),
                first: self.first,
                after: self.after.clone(),
                ..Default::default()
            },
            ..RequestBody::new(&self.operation_name) // ..Default::default()
        };
    }
}

#[cfg(test)]
mod tests {}
