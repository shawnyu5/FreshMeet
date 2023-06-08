use crate::components::meetup::Meetup;
use leptos::*;

/// fetches tech events from the meetup api
#[component]
pub fn TechEvents(cx: Scope) -> impl IntoView {
    let (queries, _) = create_signal(
        cx,
        vec![
            "tech events".to_string(),
            "programming".to_string(),
            "coding".to_string(),
        ],
    );

    view! {cx, <Meetup queries=queries/>}
}
