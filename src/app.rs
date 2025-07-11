use leptos::task::spawn_local;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use tauri_sys::{core::invoke, event::listen};
use futures_util::stream::StreamExt;

#[derive(Serialize, Deserialize)]
struct StartArgs {
    name: String,

}

#[component]
pub fn App() -> impl IntoView {
    let (clip, set_clip) = signal(String::from(""));
    let (err, set_err) = signal(String::from(""));
    let (con, set_con) = signal(String::from(""));
    let (enabled, set_enabled) = signal(false);


    spawn_local(async move {
        let mut events = listen::<String>("clip").await.unwrap();
        while let Some(event) = events.next().await {
            *set_clip.write() = event.payload;
        }
    });

    spawn_local(async move {
        let mut events = listen::<String>("err").await.unwrap();
        while let Some(event) = events.next().await {
            *set_err.write() = event.payload;
        }
    });

    spawn_local(async move {
        let mut events = listen::<String>("con").await.unwrap();
        while let Some(event) = events.next().await {
            *set_con.write() = event.payload;
        }
    });

    view! {
        <p>Current clipboard:</p>
        <textarea disabled>{clip}</textarea>
        <p>{if con.get() == "1".to_string() || con.get() == "".to_string() {
            "You're currently alone".to_string()
        } else {
            format!("There are {} people connected", con.get())
        }}</p>
        <button class:enabled=move || enabled.get()
        on:click=move |_| spawn_local(async move {
            invoke::<()>("toggle", ()).await;
            *set_enabled.write() = !enabled.get()
        })/>
        <div class="err" class:hidden=move || err.get() == "".to_string()>
        <h1>
        Error
        </h1>
        <p>
        "There was an error with the "{err}
        </p>
        <p>
        "I could've implemented better error handeling, but asking you to restart seems easier."
        </p>
        </div>
    }
}