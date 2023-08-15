use super::request::{RequestBody, Variables};

/// Builder for building a meetup request
#[derive(Default, Debug)]
pub struct RequestBuilder {
    pub query: Option<String>,
    pub first: i32,
    pub after: Option<String>,
}

impl RequestBuilder {
    /// construct a new request builder
    pub fn new() -> RequestBuilder {
        return RequestBuilder::default();
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
    pub fn build(&self) -> RequestBody {
        return RequestBody {
            variables: Variables {
                query: self.query.clone(),
                first: self.first,
                after: self.after.clone(),
                ..Default::default()
            },
            ..Default::default()
        };
    }
}
