use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::collections::HashMap;
use std::{env, error::Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pagination: Pagination,
    events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    object_count: i32,
    continuation: Option<String>,
    page_count: i32,
    page_size: i32,
    has_more_items: bool,
    page_number: i32,
}

#[derive(Serialize, Deserialize, Debug)]
/// event_sales_status object
///
/// * `event_sales_status`: information about the event sales
/// * `timezone`: timezone of the event
/// * `id`: eventbrite id of the event
/// * `privacy_setting`: uhlocked | locked
/// * `primary_organizer_id`: id of the primary organizer
/// * `tickets_url`: url to buy tickets
/// * `start_date`: start date of event
/// * `end_time`: end time of event
/// * `status`: live | virtual
pub struct Event {
    event_sales_status: EventSalesStatus,
    timezone: String,
    id: String,
    privacy_setting: String,
    primary_organizer_id: String,
    tickets_url: String,
    start_date: String,
    end_time: String,
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// event_sales_status object
///
/// * `start_sales_date`: start_sales_date object
/// * `currency`: currency of tickets
/// * `message_code`:
/// * `message`:
/// * `message_type`:
pub struct EventSalesStatus {
    start_sales_date: Option<StartSalesDate>,
    currency: String,
    message_code: Option<String>,
    message: Option<String>,
    message_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
/// start_sales_date object
///
/// * `timezone`: the timezone of the start sales date
/// * `local`: local time
/// * `utc`: time in UTC
pub struct StartSalesDate {
    timezone: String,
    local: String,
    utc: String,
}

impl Info {
    /// create a default instance of Search object
    pub fn new() -> Info {
        return Info {
            pagination: Pagination {
                object_count: 0,
                continuation: None,
                page_count: 0,
                page_size: 0,
                has_more_items: false,
                page_number: 0,
            },
            events: Vec::new(),
        };
    }

    pub async fn fetch(&self, event_ids: Vec<String>) -> Result<Info, String> {
        // event_ids=543298208567,518737516877,442445665897,534539430827,544868204467,529494461187,558994145537,538081445087,566705430197,490571601867,500297331787,482660429337,525130578697,398598979277,510949693287,483761693247,500675482847,524192553037,501785111777,500707007137&
        // ?expand=event_sales_status,primary_venue,ticket_availability,primary_organizer,public_collections&page_size=20

        let mut str_event_ids = "".to_string();
        event_ids.iter().for_each(|x| {
            str_event_ids.push_str(x);
            str_event_ids.push_str(",");
        });

        let url = "https://www.eventbrite.ca/api/v3/destination/events/";
        let query = vec![
            ("expand", "event_sales_status,image,primary_venue,saves,ticket_availability,primary_organizer,public_collections"),
            ("page_size", "20"),
            ("event_ids", str_event_ids.as_str())
        ];
        let client = reqwest::Client::new();

        let res = client
            .get(url)
            .query(&query)
            .send()
            .await
            .unwrap()
            .json::<Info>()
            .await
            .unwrap();

        let pretty = to_string_pretty(&res).unwrap();
        println!("{}", pretty);

        return Ok(res);
    }
}
