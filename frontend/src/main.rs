use std::collections::HashMap;

use dotenv::dotenv;
use leptos::*;
use leptos_router::*;
use reqwest_wasm::Client;

#[component]
fn App(cx: Scope) -> impl IntoView {
    log::info!("hello world");
    view! {cx,
      <Router>
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

#[component]
fn TechEvents(cx: Scope) -> impl IntoView {
    let data = create_resource(
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

            let res = Client::new()
                .post("http://localhost:8000/meetup/search")
                .json(&map)
                .send()
                .await
                .unwrap();
            return res.text().await.unwrap();
        },
    );
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <Button label="Hello".to_string() />
            <TechEvents/>
        </div>
    }
}

fn main() {
    dotenv().ok();
    mount_to_body(|cx| view! { cx,  <App/> });
}
