use crate::error::Error;
use crate::routes::Route;
use crate::types::{Key, PrimaryKey, User};
use crate::{components::notifier::notify, types::Notification};

use serde::de::DeserializeOwned;
use serde::Serialize;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::services::requests::{delete, post};

use super::handle_unauthorized;
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

pub fn onload_all<T: DeserializeOwned + 'static>(
    url: String,
    notify_dispatch: Dispatch<Notification>,
    history: AnyHistory,
    types: UseStateHandle<Vec<T>>,
) {
    wasm_bindgen_futures::spawn_local(async move {
        match get::<Vec<T>>(url).await {
            Ok(t) => types.set(t),
            Err(e) => match e {
                Error::Unauthorized => handle_unauthorized(history, notify_dispatch),
                _ => notify(&notify_dispatch, e.to_string(), "error".into()),
            },
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
