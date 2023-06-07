use leptos::{
    html::{p, P},
    *,
};
use networking_accumlator_types::meetup::{search_request::Result_, search_response::Response};
use reqwest_wasm::Client;
use std::collections::HashMap;

use crate::environment;

/// create a `p` tag with the description as the inner html
///
/// * `description`: the description of an event
fn format_description(cx: Scope, description: String) -> HtmlElement<P> {
    // let description = description.replace("\n", "<br>");
    let description = markdown::to_html(description.as_str());
    p(cx).inner_html(description.clone())
}

/// fetch events from the meetup api
///
/// * `page_number`:
/// * `after`:
async fn fetch_events<'a>(
    query: &'a str,
    page_number: u32,
    after: RwSignal<HashMap<String, String>>,
) -> Response {
    let env = environment::load();

    // so going back to first page will still work
    if page_number == 1 {
        match after.get().get_mut(query) {
            Some(e) => e.clear(),
            None => (),
        };
    }

    let mut map = HashMap::new();
    let mut empty_string = "".to_string();

    let mut after_value = after.get();
    // since we are making an api request, after can not be None
    let after_value = after_value.get_mut(query).unwrap_or(&mut empty_string);

    map.insert("query", query);
    map.insert("after", after_value);
    map.insert("per_page", "20");

    let events = Client::new()
        .post(format!("{}/meetup/search", &env.api_url))
        .json(&map)
        .send()
        .await
        .unwrap()
        .json::<Response>()
        .await
        .unwrap();

    after.update(|a| {
        a.insert(
            query.to_string(),
            events.page_info.endCursor.clone().unwrap_or_default(),
        );
    });
    return events;
}

/// fetches tech events from the meetup api
#[component]
pub fn TechEvents(cx: Scope) -> impl IntoView {
    // map of query string to end cursor
    let after = create_rw_signal(cx, HashMap::<String, String>::new());
    let page_number = use_context::<RwSignal<u32>>(cx).expect("a u32 read signal for page number");

    let data: Resource<u32, Vec<Result_>> = create_resource(cx, page_number, move |_| async move {
        let mut events = Vec::<Result_>::new();

        let mut responses = Vec::<Response>::new();
        responses.push(fetch_events("tech events", page_number.get(), after).await);
        responses.push(fetch_events("programming", page_number.get(), after).await);
        responses.push(fetch_events("coding", page_number.get(), after).await);

        responses.iter().for_each(|r| {
            r.nodes.iter().for_each(|e| {
                events.push(e.clone());
            });
        });

        events.sort();
        // events.iter().for_each(|e| {
        // log!("{}", e.title);
        // });
        events.dedup();
        events.sort_by(|a, b| a.dateTime.cmp(&b.dateTime));
        // log!("AFTER DE-DUPING");
        // events.iter().for_each(|e| {
        // log!("{}", e.title);
        // });
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
                        <p><b>"Time: "</b>{event.dateTime}</p>
                        { format_description(cx, event.description.clone()) }
                    </div>
                        </ErrorBoundary>
                }).collect_view(cx)
            }
            </Suspense>
    }
}
