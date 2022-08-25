use crate::routes::Route;
use crate::types::{Key, PrimaryKey, User};
use crate::{components::notifier::notify, types::Notification};
use web_sys::{HtmlInputElement, HtmlOptionElement, HtmlSelectElement};

use serde::Serialize;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::services::requests::{delete, post};

use super::requests::get;

pub fn onsubmit<T: Serialize + 'static>(
    path: String,
    body: T,
    dispatch: Dispatch<Notification>,
    history: AnyHistory,
    next_route: Route,
) -> Callback<FocusEvent> {
    Callback::once(move |e: FocusEvent| {
        e.prevent_default();
        wasm_bindgen_futures::spawn_local(async move {
            match post(path, body).await {
                Ok(data) => {
                    notify(&dispatch, data, "info".to_string());
                    history.push(next_route)
                }
                Err(e) => {
                    let error_message = format!("{:?}", e);
                    notify(&dispatch, error_message, "error".to_string());
                }
            };
        })
    })
}

pub fn ondelete(
    path: String,
    dispatch: Dispatch<Notification>,
    history: AnyHistory,
    next_route: Route,
) -> Callback<MouseEvent> {
    Callback::once(move |e: MouseEvent| {
        e.prevent_default();
        wasm_bindgen_futures::spawn_local(async move {
            match delete(path).await {
                Ok(data) => {
                    notify(&dispatch, data, "info".to_string());
                    history.push(next_route)
                }
                Err(e) => {
                    let error_message = format!("{:?}", e);
                    notify(&dispatch, error_message, "error".to_string());
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

pub fn oninput_option(state: UseStateHandle<Vec<String>>) -> Callback<MouseEvent> {
    Callback::from(move |e: MouseEvent| {
        let mut options: Vec<String> = state.iter().map(|v| v.to_owned()).collect();
        let input: HtmlOptionElement = e.target_unchecked_into();

        if input.selected() {
            if options.iter().all(|o| o != &input.value()) {
                options.push(input.value());
                state.set(options)
            }
        } else {
            options = options
                .iter()
                .filter(|o| *o != &input.value())
                .map(|o| o.to_string())
                .collect();
            state.set(options)
        }
    })
}

pub fn oninput_select(state: UseStateHandle<Vec<String>>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
            let collection = input.selected_options();
            let selected: Vec<String> = (0..input.selected_options().length())
                .filter_map(|i| collection.item(i))
                .filter_map(|e| e.text_content())
                .collect();

            state.set(selected);
        }
    })
}

pub fn get_options(
    users: UseStateHandle<Vec<String>>,
    keys: UseStateHandle<Vec<String>>,
    notify_dispatch: Dispatch<Notification>,
) {
    wasm_bindgen_futures::spawn_local(async move {
        match get::<Vec<User>>("/api/users".into()).await {
            Ok(u) => users.set(make_list(u)),
            Err(e) => notify(&notify_dispatch, e.to_string(), "error".into()),
        };
        match get::<Vec<Key>>("/api/keys?active=true".into()).await {
            Ok(k) => keys.set(make_list(k)),
            Err(e) => notify(&notify_dispatch, e.to_string(), "error".into()),
        }
    })
}

fn make_list<T: PrimaryKey>(types: Vec<T>) -> Vec<String> {
    types.iter().map(|t| t.primary_key().clone()).collect()
}
