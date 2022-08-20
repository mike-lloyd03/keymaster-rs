use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::use_store;

use crate::routes::Route;

use super::notifier::{notify, Notification};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct UserInfo {
    pub username: Option<String>,
    pub is_auth: bool,
    pub is_admin: bool,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

/// User context provider.
#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_store = use_store::<BasicStore<UserInfo>>();

    // Get session info
    {
        let notify_store = use_store::<BasicStore<Notification>>();
        let history = use_history().unwrap();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::get("/api/session").send().await.unwrap();

                    if resp.ok() {
                        match resp.json::<UserInfo>().await {
                            Ok(u) => user_store.dispatch().reduce(move |s| {
                                s.username = u.username;
                                s.is_auth = u.is_auth;
                                s.is_admin = u.is_admin;
                            }),
                            Err(e) => log::error!("{}", e),
                        }
                    } else if resp.status() == 401 {
                        history.push(Route::Login);
                    } else {
                        let resp_text = resp.text().await;
                        notify(notify_store, resp_text.unwrap(), "error".to_string());
                    };
                });
                || ()
            },
            (),
        );
    }

    html! {
        <>
            { for props.children.iter() }
        </>
    }
}
