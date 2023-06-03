use dotenv::dotenv;
use leptos::*;
use leptos_router::*;
use reqwest_wasm::Client;
use std::collections::HashMap;
use types::meetup::search_response::Response;
mod environment;

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

/// scrolls to the top of the page
fn scroll_to_top() {
    let window = web_sys::window().expect("Failed to access window object");
    window.scroll_to_with_scroll_to_options(
        web_sys::ScrollToOptions::new()
            .top(0.0)
            .behavior(web_sys::ScrollBehavior::Smooth),
    )
    // .expect("Failed to scroll to top");
}
/// fetches tech events from the meetup api
#[component]
fn TechEvents(cx: Scope) -> impl IntoView {
    let after = use_context::<RwSignal<String>>(cx).expect("a String read write signal for after");
    let page_number = use_context::<RwSignal<u32>>(cx).expect("a u32 read signal for page number");

    let data: Resource<u32, Response> = create_resource(cx, page_number, move |_| async move {
        let env = environment::load();

        let mut map = HashMap::new();
        let after_value = after.get();
        let after_value = after_value.as_str();

        map.insert("query", "tech");
        map.insert("after", after_value);
        map.insert("per_page", "10");

        let events = Client::new()
            .post(format!("{}/meetup/search", &env.api_url))
            .json(&map)
            .send()
            .await
            .unwrap()
            .json::<Response>()
            .await
            .unwrap();

        after.set(events.page_info.endCursor.clone().unwrap_or_default());
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
            <button><A href="/">"Tech events"</A></button>
            <button><A href="/about">"About"</A></button>
        </div>
    }
}

/// the home page
#[component]
fn Home(cx: Scope) -> impl IntoView {
    let page_signal = create_rw_signal(cx, 1 as u32);
    let after_signal = create_rw_signal(cx, "".to_string());
    provide_context(cx, page_signal);
    provide_context(cx, after_signal);

    create_effect(cx, move |_| {
        log!("page number changed to {}", page_signal.get());
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
    let page_signal = use_context::<RwSignal<u32>>(cx).expect("a write signal");

    view! { cx,
    <div>
        <button
            on:click = move |_| page_signal.update(|value| {
                if *value != 1 {
                    *value -= 1 as u32;
                    scroll_to_top();
                }
            })
            >
            "Previous"
        </button>

        <button
            on:click = move |_| page_signal.update(|value| {
                *value += 1 as u32;
                scroll_to_top();
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
