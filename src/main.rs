use dotenv::dotenv;
mod eventbrite;
use serde_json::to_string_pretty;
mod meetup;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let search = meetup::search::Search::new();

    // event_ids=543298208567,518737516877,442445665897,534539430827,544868204467,529494461187,558994145537,538081445087,566705430197,490571601867,500297331787,482660429337,525130578697,398598979277,510949693287,483761693247,500675482847,524192553037,501785111777,500707007137&
    let event_ids = vec![
        "543298208567".to_string(),
        "518737516877".to_string(),
        "442445665897".to_string(),
        "534539430827".to_string(),
        "544868204467".to_string(),
        "529494461187".to_string(),
        "558994145537".to_string(),
        "538081445087".to_string(),
        "566705430197".to_string(),
        "490571601867".to_string(),
        "500297331787".to_string(),
        "482660429337".to_string(),
        "525130578697".to_string(),
        "398598979277".to_string(),
        "510949693287".to_string(),
        "483761693247".to_string(),
        "500675482847".to_string(),
        "524192553037".to_string(),
        "501785111777".to_string(),
        "500707007137".to_string(),
    ];

    let result = search.fetch().await;
    println!("{}", to_string_pretty(&result).unwrap());
    // let result = user.user_info().await.unwrap();
    // println!("{:?}", result);

    // let response =
    // reqwest::blocking::get("https://www.eventbrite.ca/d/canada--toronto/tech-events/")
    // .unwrap()
    // .text()
    // .unwrap();

    // let document = scraper::Html::parse_document(&response);
    // let title_selector =
    // scraper::Selector::parse(".eds-event-card-content__content-container eds-l-pad-right-4")
    // .unwrap();
    // let cards = document.select(&title_selector).map(|x| x.inner_html());
    // cards
    // .zip(1..101)
    // .for_each(|(item, number)| println!("{}. {}", number, item));
}
