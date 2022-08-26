use crate::components::notifier::{notify_error, notify_info};
use crate::services::auth::{current_user, login_user};
use crate::types::{Credentials, SessionInfo};
use crate::{
    components::form::{Button, ButtonType, Form, PasswordField, TextField},
    services::auth::logout_user,
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use super::Route;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(String::new);
    let password = use_state(String::new);

    let onsubmit = {
        let creds = Credentials {
            username: (*username).clone(),
            password: (*password).clone(),
        };
        let history = use_history().unwrap();
        Callback::from(move |e: FocusEvent| {
            notify_info("Logging in...");
            let creds = creds.clone();
            let history = history.clone();
            e.prevent_default();
            wasm_bindgen_futures::spawn_local(async move {
                login_user(creds, &history).await;
            })
        })
    };

    html! {
        <Form title="Login" {onsubmit}>
            <TextField label="Username" required=true state={username}/>
            <PasswordField label="Password" state={password}/>
            <Button value="Login" button_type={ButtonType::Primary} />
        </Form>
    }
}

#[function_component(Logout)]
pub fn logout() -> Html {
    wasm_bindgen_futures::spawn_local(async move {
        if current_user().is_auth {
            logout_user().await;
        }
    });

    html! {
        <Redirect<Route> to={Route::Login} />
    }
}

#[derive(Properties, PartialEq)]
pub struct ChildrenProps {
    pub admin: Option<bool>,
    pub children: Children,
}

#[function_component(CheckAuth)]
pub fn check_auth(props: &ChildrenProps) -> Html {
    let (session, _) = use_store::<SessionInfo>();

    log::info!("Mounting CheckAuth");

    if session.is_auth {
        match props.admin {
            Some(true) => {
                if session.is_admin {
                    html! {
                        {for props.children.iter()}
                    }
                } else {
                    notify_error("You must be an administrator to access this page.");
                    html! {
                        <Redirect<Route> to={Route::Home}/>
                    }
                }
            }
            _ => html! {
                {for props.children.iter()}
            },
        }
    } else {
        // notify_error("You must login to access this page.");
        html! {
            <Login />
        }
    }
}
