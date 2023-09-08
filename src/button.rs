use::leptos::*;

#[component]
pub fn Button(cx: Scope) -> impl IntoView {
    view! { cx, <button class="px-4 py-2 rounded-md text-white bg-indigo-500">"Click me"</button> }
}
