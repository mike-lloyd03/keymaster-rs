use crate::components::notifier::{notify, Notification};
use crate::routes::Route;
use web_sys::HtmlInputElement;

use serde::Serialize;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::StoreRef;

use crate::services::requests::{delete, post};

pub fn onsubmit<T: Serialize + 'static>(
    path: String,
    body: T,
    store: StoreRef<BasicStore<Notification>>,
    history: AnyHistory,
    next_route: Route,
) -> Callback<FocusEvent> {
    Callback::once(move |e: FocusEvent| {
        e.prevent_default();
        wasm_bindgen_futures::spawn_local(async move {
            match post(path, body).await {
                Ok(data) => {
                    notify(store, data, "info".to_string());
                    history.push(next_route)
                }
                Err(e) => {
                    let error_message = format!("{:?}", e);
                    notify(store, error_message, "error".to_string());
                }
            };
        })
    })
}

pub fn ondelete(
    path: String,
    store: StoreRef<BasicStore<Notification>>,
    history: AnyHistory,
    next_route: Route,
) -> Callback<MouseEvent> {
    Callback::once(move |e: MouseEvent| {
        e.prevent_default();
        wasm_bindgen_futures::spawn_local(async move {
            match delete(path).await {
                Ok(data) => {
                    notify(store, data, "info".to_string());
                    history.push(next_route)
                }
                Err(e) => {
                    let error_message = format!("{:?}", e);
                    notify(store, error_message, "error".to_string());
                }
            };
        })
    })
}

pub fn oninput_string(state: UseStateHandle<String>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        state.set(input.value());
    })
}

pub fn oninput_bool(state: UseStateHandle<bool>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        state.set(input.checked());
    })
}
