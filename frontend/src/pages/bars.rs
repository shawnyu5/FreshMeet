use leptos::*;

use crate::components::meetup::Meetup;

#[component]
pub fn Bars(cx: Scope) -> impl IntoView {
    let (queries, _) = create_signal(cx, vec!["bars".to_string(), "food".to_string()]);

    // let filter_fn = |e: &Result_| e == e;
    view! {cx, <Meetup queries=queries/>}
}
