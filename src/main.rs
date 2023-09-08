mod components;
use leptos::*;
use components::button::Button;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
      <main>
        <Button/>
        <div>"Hello, world!"</div>
      </main>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}