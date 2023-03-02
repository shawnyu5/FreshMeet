use dotenv::dotenv;
mod eventbrite;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // let eventbrite = eventbrite::EventBrite::new();
    let user = eventbrite::user::new();

    let result = user.user_info().await.unwrap();
    println!("{:?}", result);

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
