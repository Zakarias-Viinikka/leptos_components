use std::time::Duration;

use leptos::prelude::*;

/*struct PopupAndId {
    popup: Popup,
    id: i32,
}*/

#[component]
pub fn Popup(begone_wordly_desire: Callback<()>, what_to_say: String) -> impl IntoView {
    let (timer, timer_s) = signal(0);

    Effect::new(move |_| {
        let handle = set_interval_with_handle(
            move || tick_the_timer(timer_s, begone_wordly_desire),
            Duration::from_millis(500),
        )
        .expect("failed to set interval");
        on_cleanup(move || handle.clear());
    });
    view! {
        <div>
            {what_to_say}
            <br/>
            {move || timer.get()}
        </div>
    }
}

fn tick_the_timer(timer_s: WriteSignal<i32>, begone_wordly_desire: Callback<()>) {
    let mut should_remove = false;
    timer_s.update(|t| {
        if *t >= 10 {
            should_remove = true;
        } else {
            *t += 1;
        }
    });
    if should_remove {
        begone_wordly_desire.run(());
    }
}

//pub fn spawn_popup() -> impl IntoAny {}
