use dotenv::dotenv;
use leptos::*;
use leptos_router::*;
use reqwest_wasm::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::meetup::search_response::Response;

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

/// fetches events from the meetup api
#[component]
fn TechEvents(cx: Scope) -> impl IntoView {
    let page_number = use_context::<ReadSignal<u32>>(cx).expect("a read signal");

    let data: Resource<u32, Response> =
        create_resource(cx, page_number.clone(), move |_| async move {
            let page_number = page_number.get().to_string();
            let page_number = page_number.as_str();

            log!("fetching data");
            let mut map = HashMap::new();
            map.insert("query", "tech");
            map.insert("page", page_number);
            map.insert("per_page", "10");
            map.insert("start_date", "2023-05-31T00:00:00");

            let events = Client::new()
                .post("http://localhost:8000/meetup/search")
                .json(&map)
                .send()
                .await
                .unwrap()
                .json::<Response>()
                .await
                .unwrap();

            return events;
        });

    fn format_description(description: String) -> Vec<String> {
        return description.split("\n").map(|s| s.to_string()).collect();
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
                                                      format_description(event.description.clone())
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

/// the home page
#[component]
fn Home(cx: Scope) -> impl IntoView {
    let (page_number, set_page_number) = create_signal(cx, 2 as u32);
    provide_context(cx, page_number);
    provide_context(cx, set_page_number);

    create_effect(cx, move |_| {
        log!("page number changed to {}", page_number.get());
    });
    // pass to components page number, and container of events.
    // every time page number changes, update events

    view! { cx,
        <div>
            <TechEvents/>
            <Pagination/>
        </div>
    }
}

/// pagination buttons
///
/// * `page_number`: the current page number
#[component]
fn Pagination(cx: Scope) -> impl IntoView {
    let set_page_number = use_context::<WriteSignal<u32>>(cx).expect("a write signal");

    view! { cx,
    <div>
        <button
            on:click = move |_| set_page_number.update(|value| {
                if *value != 1 {
                    *value -= 1 as u32;
                }

            })
            >
            "Previous"
        </button>

        <button
            on:click = move |_| set_page_number.update(|value| {
                *value += 1 as u32;
            })
            >
            "Next"
        </button>

    </div>
    }
}

fn main() {
    dotenv().ok();
    mount_to_body(|cx| view! { cx,  <App/> });
}
