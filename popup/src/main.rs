use leptos::prelude::*;
use popup::popup::Popup;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

macro_rules! for_leptos {
    ($list:expr, $item:ident => $body:expr) => {
        view! {
            <For
                each=move || $list.get()
                key=|$item| $item.id
                children=move |$item| $body
            />
        }
    };
}

#[derive(Clone)]
struct PopupSpawner {
    what_to_say: String,
    id: i32,
}

#[component]
fn App() -> impl IntoView {
    let (popup_spawner, popup_spawner_s) = signal(Vec::<PopupSpawner>::new());
    let (ctr, ctr_s) = signal(0);
    view! {
        <button on:click=move |_| {
            popup_spawner_s.update(|list| {
                ctr_s.set(ctr.get() + 1);
                list.push(PopupSpawner {
                    what_to_say: "popup 1".to_string(),
                    id: ctr.get(),
                });
            });
        }>"Create popup"</button>
        <button on:click=move |_| {
            popup_spawner_s.update(|list| {
                ctr_s.set(ctr.get() + 1);
                list.push(PopupSpawner {
                    what_to_say: "popup 2".to_string(),
                    id: ctr.get(),
                });
            });
        }>"Create different popup"</button>

        {for_leptos!(popup_spawner, popup => {
            view! {
                <Popup
                    begone_wordly_desire=Callback::new(move |_| remove_me(popup_spawner_s, popup.id))
                    what_to_say=popup.what_to_say.clone()
                />
            }
        })}
    }
}

fn remove_me(popup_spawner_s: WriteSignal<Vec<PopupSpawner>>, id: i32) {
    popup_spawner_s.update(|list| list.retain(|item| item.id != id));
}
