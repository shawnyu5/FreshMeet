use std::collections::HashMap;

use leptos::*;
use networking_accumlator_types::meetup::search_response::Response;
use reqwest_wasm::Client;

use crate::environment;

/// format the description of an event
///
/// * `description`: the description of an event
fn format_description(description: String) -> Vec<String> {
    return description.split("\n").map(|s| s.to_string()).collect();
}

/// fetches tech events from the meetup api
#[component]
pub fn TechEvents(cx: Scope) -> impl IntoView {
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

    view! {
        cx,
         <Suspense
            fallback=move || view! { cx, <p>"Loading..."</p> }
            >
            {
                data.read(cx).unwrap_or_default().into_iter().map(|event| view! {cx,
                    <ErrorBoundary
                        fallback=|cx, errors| view! {cx,
                        <div>
                            <p>"Error rendering event: "</p>
                            <ul>
                            {move || errors.get()
                                .into_iter()
                                    .map(|(_, e)| view! {cx, <li>{e.to_string()}</li>})
                                    .collect_view(cx)}
                        </ul>
                            </div>
                        }
                    >
                        <div>
                        <h3>{event.title.clone()}</h3>
                        {
                            format_description(event.description.clone())
                                .into_iter()
                                .map(|d| view! { cx, <p>{d}</p>})
                                .collect_view(cx)
                        }
                    </div>
                        </ErrorBoundary>
                }).collect_view(cx)
            }
            </Suspense>
    }
}
