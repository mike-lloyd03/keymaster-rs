use crate::components::notifier::Notification;
use gloo_net::http::Request;
use serde_json::json;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::StoreRef;

mod assignments;
mod auth;
mod keys;
mod users;

#[derive(Clone, Routable, PartialEq)]
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
    #[at("/404")]
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

pub fn onsubmit<T: serde::Serialize + 'static + Clone>(
    path: &'static str,
    fields: Vec<(&'static str, UseStateHandle<T>)>,
    store: StoreRef<BasicStore<Notification>>,
    history: AnyHistory,
) -> Callback<FocusEvent> {
    let mut map = std::collections::HashMap::new();
    for f in fields {
        map.insert(f.0, (*f.1).clone());
    }

    Callback::once(move |e: FocusEvent| {
        e.prevent_default();
        wasm_bindgen_futures::spawn_local(async move {
            let resp = Request::post(path)
                .json(&json!(map))
                .unwrap()
                .send()
                .await
                .unwrap();

            let resp_text = resp.text().await;
            match resp.ok() {
                false => store.dispatch().reduce(|s| {
                    s.msg = Some(resp_text.unwrap());
                    s.lvl = Some("error".to_string());
                }),
                true => {
                    store.dispatch().reduce(|s| {
                        s.msg = Some(resp_text.unwrap());
                        s.lvl = Some("info".to_string());
                    });
                    history.push(Route::Keys)
                }
            }
        })
    })
}

pub fn oninput(state: UseStateHandle<String>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        state.set(input.value());
    })
}
