use dotenv::dotenv;
mod pages;
use leptos::*;
use leptos_router::*;
use pages::tech_events::*;
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
    // let after_signal = create_rw_signal(cx, "".to_string());
    provide_context(cx, page_signal);
    // provide_context(cx, after_signal);

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

/// scrolls to the top of the page
fn scroll_to_top() {
    let window = web_sys::window().expect("Failed to access window object");
    window.scroll_to_with_scroll_to_options(
        web_sys::ScrollToOptions::new()
            .top(0.0)
            .behavior(web_sys::ScrollBehavior::Smooth),
    )
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
    mount_to_body(|cx| view! { cx, <App/> });
}
