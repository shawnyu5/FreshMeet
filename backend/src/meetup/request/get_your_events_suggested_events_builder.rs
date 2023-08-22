use super::{
    common::{EventType, OperationName},
    get_your_events_suggested_events::{GetYourEventsSuggestedEventsRequest, Variables},
};

/// Builder for building a meetup request
#[derive(Debug, Default)]
pub struct GetYourEventsSuggestedEventsBuilder {
    first: Option<u32>,
    event_type: Option<EventType>,
}

impl GetYourEventsSuggestedEventsBuilder {
    pub fn new() -> Self {
        GetYourEventsSuggestedEventsBuilder::default()
    }
    pub fn event_type(&mut self, event_type: EventType) -> &mut Self {
        self.event_type = Some(event_type);
        return self;
    }

    pub fn first(&mut self, first: u32) -> &mut Self {
        self.first = Some(first);
        return self;
    }

    pub fn build(&mut self) -> GetYourEventsSuggestedEventsRequest {
        let variables = Variables {
            first: self.first.unwrap_or(30),
            eventType: self
                .event_type
                .as_ref()
                .unwrap_or_else(|| &EventType::physical)
                .to_string(),
            ..Default::default()
        };
        return GetYourEventsSuggestedEventsRequest {
            operationName: OperationName::getYourEventsSuggestedEvents.to_string(),
            variables,
            ..Default::default()
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// able to set first on a request
    #[test]
    fn can_set_first() {
        let request = GetYourEventsSuggestedEventsBuilder::new().first(30).build();
        assert_eq!(request.variables.first, 30);
    }

    /// able to set event type on a request
    #[test]
    fn can_set_event_type() {
        let request = GetYourEventsSuggestedEventsBuilder::new()
            .event_type(EventType::online)
            .build();
        assert_eq!(request.variables.eventType, EventType::online.to_string());
    }
}
