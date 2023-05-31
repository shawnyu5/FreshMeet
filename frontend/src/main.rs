use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <p>"Hello, world!"</p>
    }
}

#[component]
fn Counter(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    view! {
        cx,
        <button
            on:click=move |_| set_count(count.get() + 1)
            >{ count }</button>
    }
}
fn main() {
    mount_to_body(|cx| view! { cx,  <Counter /> });
}
