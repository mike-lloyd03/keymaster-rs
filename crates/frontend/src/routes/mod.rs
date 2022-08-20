use crate::components::notifier::{notify, Notification};
use crate::services::requests::{delete, post};

use serde::Serialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::StoreRef;

mod assignments;
mod auth;
mod keys;
mod users;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/logout")]
    Logout,
    #[at("/keys")]
    Keys,
    #[at("/add-key")]
    AddKey,
    #[at("/edit-key/:key_name")]
    EditKey { key_name: String },
    #[at("/assignments")]
    Assignments,
    #[at("/assign-key")]
    AssignKey,
    #[at("/edit-assignment")]
    EditAssignment,
    #[at("/users")]
    Users,
    #[at("/add-user")]
    AddUser,
    #[at("/edit-user")]
    EditUser,
    #[not_found]
    #[at("/not-found")]
    NotFound,
}

#[function_component(Home)]
fn home() -> Html {
    html! {
        <div>
            <h1>{ "This is home" }</h1>
        </div>
    }
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <auth::Login /> },
        Route::Logout => html! { <auth::Logout /> },
        Route::Keys => html! { <keys::KeyTable /> },
        Route::AddKey => html! { <keys::NewKey />},
        Route::EditKey { key_name } => html! { <keys::EditKey key_name={ key_name.clone() }/>},
        Route::Assignments => html! { <assignments::Assignments />},
        Route::AssignKey => html! { <assignments::NewAssignment />},
        Route::EditAssignment => html! { <assignments::EditAssignment />},
        Route::Users => html! { <users::UserTable /> },
        Route::AddUser => html! { <users::NewUser />},
        Route::EditUser => html! { <users::EditUser />},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

pub fn onsubmit<T: Serialize + 'static>(
    path: String,
    json: T,
    store: StoreRef<BasicStore<Notification>>,
    history: AnyHistory,
    next_route: Route,
) -> Callback<FocusEvent> {
    Callback::once(move |e: FocusEvent| {
        e.prevent_default();
        wasm_bindgen_futures::spawn_local(async move {
            match post(path, json).await {
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
