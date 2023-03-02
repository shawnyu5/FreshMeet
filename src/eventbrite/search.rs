use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, error::Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct Search {
    event_search: EventSearch,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventSearch {
    q: String,
    dates: String,
    dedup: bool,
    places: Vec<String>,
    page: i32,
    page_size: i32,
    online_events_only: bool,
    include_promoted_events_for: IncludePromotedEventsFor,
    #[serde(rename(
        serialize = "expand.destination_event",
        deserialize = "expand.destination_event"
    ))]
    expand_destination_event: Vec<String>,
    // dont deserialize or serialize this field
    #[serde(skip_serializing)]
    debug_experiment_overrides: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncludePromotedEventsFor {
    interface: String,
    request_source: String,
}
// {"event_search":{"q":"tech events","dates":"current_future","dedup":true,"places":["101735835"],"page":1,"page_size":20,"online_events_only":false,"include_promoted_events_for":{"interface":"search","request_source":"web"}},"expand.destination_event":["primary_venue","image","ticket_availability","saves","event_sales_status","primary_organizer","public_collections"],"debug_experiment_overrides":{"search_exp_3":"A"}}

// {"event_search":{"q":"tech events","dates":"current_future","dedup":true,"places":["101735835"],"page":1,"page_size":20,"online_events_only":false,"include_promoted_events_for":{"interface":"search","request_source":"web"},"expand.destination_event":["primary_venue","primary_venue","image","ticket_availability","saves","event_sales_status","primary_organizer","public_collections"]}}

impl Search {
    pub fn new() -> Search {
        return Search {
            event_search: EventSearch {
                q: "tech events".to_string(),
                dates: "current_future".to_string(),
                dedup: true,
                places: vec!["101735835".to_string()],
                page: 1,
                page_size: 20,
                online_events_only: false,
                include_promoted_events_for: IncludePromotedEventsFor {
                    interface: "search".to_string(),
                    request_source: "web".to_string(),
                },
                expand_destination_event: vec![
                    "primary_venue".to_string(),
                    "primary_venue".to_string(),
                    "image".to_string(),
                    "ticket_availability".to_string(),
                    "saves".to_string(),
                    "event_sales_status".to_string(),
                    "primary_organizer".to_string(),
                    "public_collections".to_string(),
                ],
                debug_experiment_overrides: None,
            },
        };
    }

    // pub fn search() -> Result<String, Box<dyn Error>> {
    pub async fn search(&self) {
        #[derive(Serialize, Deserialize, Debug)]
        struct Body {
            event_search: EventSearch,
        }
        let url = "https://www.eventbrite.ca/api/v3/destination/search/";
        // let body = Search::new();
        // let j = serde_json::to_string(&body).unwrap();
        // println!("{}", j);

        let body = "{\"event_search\":{\"q\":\"tech events\",\"dates\":\"current_future\",\"dedup\":true,\"places\":[\"101735835\"],\"page\":1,\"page_size\":20,\"online_events_only\":false,\"include_promoted_events_for\":{\"interface\":\"search\",\"request_source\":\"web\"}},\"expand.destination_event\":[\"primary_venue\",\"image\",\"ticket_availability\",\"saves\",\"event_sales_status\",\"primary_organizer\",\"public_collections\"],\"debug_experiment_overrides\":{\"search_exp_3\":\"A\"}}";
        let client = reqwest::Client::new();
        let res = client
            .post(url)
            .json(&body)
            .header("cookie", "G=v%3D2%26i%3D53667589-8561-456a-9056-ff021cc9f445%26a%3D10e2%26s%3D3c91c4a781e20bb8b26292f11257c452f85e9ce5; eblang=lo%3Den_CA%26la%3Den-ca; AS=551feafa-1cf7-480f-990f-3abc358677b5; csrftoken=aff71306b84c11ed852d1f998e7209b9; mgrefby=\"https://www.google.com/\"; mgref=refsites; _gid=GA1.2.1640365782.1677687601; ebGAClientId=1585740576.1677687601; _gcl_au=1.1.526030219.1677687603; _scid=23f0784d-27e4-4194-99d4-df45aeda2ad8; _tt_enable_cookie=1; _ttp=19j5jfRJrfUI5HSYC3rUZRisnJ9; _pin_unauth=dWlkPU9EazRPR0UzTXpVdE5XWTNaQzAwWkRKbUxUbG1OVGd0TlRBek4yWTNOVE16WlRobQ; _sctr=1|1677646800000; hubspotutk=9a5508a87b8c873482aaf38f850d8c36; __hssrc=1; ln_or=eyI5NDQzNiI6ImQifQ%3D%3D; mgaff518737516877=ebdssbdestsearch; ajs_user_id=null; ajs_group_id=null; ajs_anonymous_id=%22ac147c92-22a9-44e9-a10a-6809acdbea97%22; _hp2_props.1404198904=%7B%7D; SS=AE3DLHQMFrekMYaAmXnWwWg19Nt8Lxubaw; _hp2_id.1404198904=%7B%22userId%22%3A%225330432376296632%22%2C%22pageviewId%22%3A%221007006494348479%22%2C%22sessionId%22%3A%221151198826619173%22%2C%22identity%22%3Anull%2C%22trackerVersion%22%3A%224.0%22%7D; _hp2_ses_props.1404198904=%7B%22ts%22%3A1677728431676%2C%22d%22%3A%22www.eventbrite.ca%22%2C%22h%22%3A%22%2Fd%2Fcanada--toronto%2Ftech-events%2F%22%2C%22q%22%3A%22%3Fpage%3D1%22%7D; _gat=1; _ga_TQVES5V6SH=GS1.1.1677728428.5.1.1677728434.0.0.0; _ga=GA1.1.1585740576.1677687601; __hstc=58577909.9a5508a87b8c873482aaf38f850d8c36.1677687605873.1677723884697.1677728434581.5; __hssc=58577909.1.1677728434581; SP=AGQgbbl1WJwd8StdaiC1sZ2B-WOqB50T2ol96Hxo_V6FTBKR4k2nFhUzqpt5LzHxBZGVxt9sQY-I5Ksx9GUW4b8-ZKlhcWrHE5qHzIQnHFXSLSvNOxnu1r2uHc5LCc90qxtdsIvHVavdxHPIinWWbvEjFy8Fyn75OoggKbrGPK3a8vrbNFqe8_38algeCblqg9_mxY_d0A7GKLNQYP3szjK3XET43Q-5VpAd17X_eVA0tPFN7fKKZDQ; _dd_s=rum=0&expire=1677729335717")
            .header(
                "referer",
                "https://www.eventbrite.ca/d/canada--toronto/tech-events/?page=1",
            )
            .header("x-csrftoken", "aff71306b84c11ed852d1f998e7209b9")
            .header("origin", "https://www.eventbrite.ca")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        println!("{:?}", res);
    }
}
