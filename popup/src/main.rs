use leptos::prelude::*;
use popup::popup;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <button on:click=move |_| {
            popup::create_popup("popup 1".to_string());
        }>"Create popup"</button>
        <button on:click=move |_| {
            popup::create_popup("popup 2".to_string());
        }>"Create different popup"</button>


        <popup::PopupContainer />
    }
}
