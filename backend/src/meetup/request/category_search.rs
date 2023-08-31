use crate::meetup::request::common::OperationName;
use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySearchRequest {
    pub operation_name: String,
    pub variables: Variables,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Variables {
    pub first: i32,
    pub lat: f64,
    pub lon: f64,
    pub sort_field: String,
    pub start_date_range: String,
    pub end_date_range: String,
}

impl Default for CategorySearchRequest {
    fn default() -> Self {
        // Get the current UTC date and time
        let utc: DateTime<Utc> = Utc::now();
        // Create a fixed offset representing -04:00 (Eastern Daylight Time)
        let offset = FixedOffset::east_opt(-4 * 3600).unwrap();
        // Convert the UTC time to the specified offset
        let local_time: DateTime<FixedOffset> = utc.with_timezone(&offset);
        // Format the local time in the desired format
        let start_time = local_time.format("%Y-%m-%dT%H:%M:%S-04:00").to_string();
        dbg!(&start_time);
        let end_time = local_time.format("%Y-%m-%dT23:59:59-04:00").to_string();

        let variables = Variables {
            first: 20,
            lat: 43.7400016784668,
            lon: -79.36000061035156,
            sort_field: "RELEVANCE".to_string(),
            start_date_range: start_time,
            end_date_range: end_time,
        };

        return Self {
            operation_name: OperationName::categorySearch.to_string(),
            variables,
        };
    }
}
