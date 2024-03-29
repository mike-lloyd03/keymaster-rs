use crate::components::notifier::{notify_error, notify_info};

use crate::routes::Route;

use crate::types::{Key, PrimaryKey, User};

use serde::de::DeserializeOwned;
use serde::Serialize;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::services::requests::{delete, post};

use super::requests::get;

pub fn submit_form<T: Clone + Serialize + 'static>(
    path: String,
    body: T,
    history: AnyHistory,
    next_route: Route,
) -> Callback<FocusEvent> {
    Callback::from(move |e: FocusEvent| {
        let path = path.clone();
        let body = body.clone();
        let history = history.clone();
        let next_route = next_route.clone();
        e.prevent_default();
        wasm_bindgen_futures::spawn_local(async move {
            match post::<T, String>(path, body).await {
                Ok(data) => {
                    notify_info(&data);
                    history.push(next_route)
                }
                Err(e) => {
                    notify_error(&e.to_string());
                }
            };
        })
    })
}

pub fn ondelete(path: String, history: AnyHistory, next_route: Route) -> Callback<MouseEvent> {
    Callback::once(move |e: MouseEvent| {
        e.prevent_default();
        wasm_bindgen_futures::spawn_local(async move {
            match delete::<String>(path).await {
                Ok(data) => {
                    notify_info(&data);
                    history.push(next_route)
                }
                Err(e) => {
                    notify_error(&e.to_string());
                }
            };
        })
    })
}

/// Gets the resources from the specified path and writes them into a state object
pub fn onload<T>(url: String, item: UseStateHandle<T>)
where
    T: DeserializeOwned + 'static,
{
    wasm_bindgen_futures::spawn_local(async move {
        match get::<T>(url).await {
            Ok(t) => item.set(t),
            Err(e) => notify_error(&e.to_string()),
        }
    })
}

pub fn get_options(users: UseStateHandle<Vec<String>>, keys: UseStateHandle<Vec<String>>) {
    wasm_bindgen_futures::spawn_local(async move {
        match get::<Vec<User>>("/api/users".into()).await {
            Ok(u) => users.set(make_list(u)),
            Err(e) => notify_error(&e.to_string()),
        };
        match get::<Vec<Key>>("/api/keys?active=true".into()).await {
            Ok(k) => keys.set(make_list(k)),
            Err(e) => notify_error(&e.to_string()),
        }
    })
}

fn make_list<T: PrimaryKey>(types: Vec<T>) -> Vec<String> {
    types.iter().map(|t| t.primary_key()).collect()
}
