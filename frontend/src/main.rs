use dotenv::dotenv;
use leptos::{html::br, *};
use leptos_router::*;
use reqwest_wasm::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! {cx,
      <Router>
          <NavBar/>
          <main>
              <Routes>
                  <Route
                      path="/"
                      view=|cx| view! { cx, <Home/> }
                  />
                  </Routes>
          </main>
      </Router>
    }
}

/// a generic button component
#[component]
fn Button(cx: Scope, label: String) -> impl IntoView {
    view! {cx,
        <div
            style="class: button"
            >
            <button>{label}</button>
        </div>
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EventType {
    physical,
    online,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum RsvpState {
    #[default]
    JOIN_OPEN,
    CLOSED,
    JOIN_APPROVAL,
    NOT_OPEN_YET,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
/// response object for /search
pub struct Response {
    page_info: PageInfo,
    nodes: Vec<Result_>,
}

impl IntoIterator for Response {
    type Item = Result_;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub data: Data,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Data {
    pub results: Results,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Results {
    pub pageInfo: PageInfo,
    pub count: i32,
    pub edges: Vec<Edge>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct PageInfo {
    pub hasNextPage: bool,
    pub endCursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Edge {
    pub node: Node,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Node {
    pub id: String,
    pub result: Result_,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
/// Details about a meetup event
pub struct Result_ {
    pub id: String,
    pub title: String,
    pub dateTime: String,
    pub endTime: String,
    pub description: String,
    pub duration: String,
    pub timezone: String,
    pub eventType: String,
    pub currency: String,
    pub eventUrl: String,
    pub going: Option<i32>,
    pub isAttending: bool,
    pub rsvpState: RsvpState,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Variables {
    pub after: String,
    pub first: i32,
    pub lat: f64,
    pub lon: f64,
    pub eventType: Option<EventType>,
    pub topicCategoryId: Option<String>,
    pub startDateRange: String,
    pub startDate: Option<String>,
    pub source: String,
    pub query: String,
    pub sortField: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub zip: String,
}

#[component]
fn TechEvents(cx: Scope) -> impl IntoView {
    let data: Resource<(), Response> = create_resource(
        cx,
        || (),
        |_| async move {
            log!("fetching data");
            let mut map = HashMap::new();
            // - `query`: the search query
            // - `page`: page number to display
            // - `per_page`: number of nodes to return
            // - `start_date`: start date of events in ISO 8601 format
            map.insert("query", "tech");
            map.insert("page", "1");
            map.insert("per_page", "10");
            map.insert("start_date", "2023-05-31T00:00:00");

            log!("sending request");

            let events = Client::new()
                .post("http://localhost:8000/meetup/search")
                .json(&map)
                .send()
                .await
                .unwrap()
                .json::<Response>()
                .await
                .unwrap();

            events
                .clone()
                .into_iter()
                .for_each(|e| log!("{}", e.description));

            return events;
        },
    );

    fn format_description(cx: Scope, description: String) -> Vec<String> {
        let des = description.split("\n").map(|s| s.to_string()).collect();

        return des;
    }

    view! {
    cx,
    { move || match data.read(cx) {
                                      None => view! {cx, <div>"loading..."</div> }.into_view(cx),
                                      Some(data) => {
                                          data.into_iter().map(|event| view! {cx,
                                              <div>
                                                  <h3>{event.title.clone()}</h3>
                                                  {
                                                      format_description(cx, event.description.clone())
                                                      .into_iter()
                                                      .map(|d| view! { cx, <p>{d}</p>})
                                                      .collect_view(cx)
                                                  }
                                              </div>
                                          }).collect_view(cx)
                                      }
                                  }
    }
    }
}

/// a navbar component that goes at the top of all pages
#[component]
fn NavBar(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <button><A href="/">"Home"</A></button>
            <button><A href="/about">"About"</A></button>
        </div>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <TechEvents/>
        </div>
    }
}

fn main() {
    dotenv().ok();
    mount_to_body(|cx| view! { cx,  <App/> });
}
