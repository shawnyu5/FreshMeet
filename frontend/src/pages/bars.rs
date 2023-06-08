use leptos::*;

use crate::components::meetup::Meetup;

#[component]
pub fn Bars(cx: Scope) -> impl IntoView {
    let (queries, _) = create_signal(cx, vec!["bars".to_string(), "food".to_string()]);

    view! {cx, <Meetup queries=queries/>}
}
