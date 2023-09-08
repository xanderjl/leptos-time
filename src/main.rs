mod button;
use leptos::*;
use button::*;

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