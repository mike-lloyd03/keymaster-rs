use crate::types::{Credentials, UserInfo};
use crate::{
    components::{
        form::{Button, ButtonType, Form, PasswordField, TextField},
        notifier::{notify, Notification},
    },
    services::requests::{get, post},
};
use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use super::Route;
use crate::services::form_actions::oninput_string;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(String::new);
    let password = use_state(String::new);
    let oninput_username = oninput_string(username.clone());
    let oninput_password = oninput_string(password.clone());

    let onsubmit = {
        let creds = Credentials {
            username: (*username).clone(),
            password: (*password).clone(),
        };
        let (_, notify_dispatch) = use_store::<Notification>();
        let (_, user_dispatch) = use_store::<UserInfo>();
        let history = use_history().unwrap();
        Callback::once(move |e: FocusEvent| {
            e.prevent_default();
            wasm_bindgen_futures::spawn_local(async move {
                match post::<Credentials, String>("/api/login".into(), creds).await {
                    Ok(_) => {
                        let ui: UserInfo = get("/api/session".into()).await.unwrap();
                        user_dispatch.reduce_mut(|s| {
                            s.username = ui.username;
                            s.is_auth = ui.is_auth;
                            s.is_admin = ui.is_admin;
                        });
                        history.push(Route::Home)
                    }
                    Err(e) => {
                        let error_message = format!("{:?}", e);
                        notify(notify_dispatch, error_message, "error".into());
                    }
                };
            })
        })
    };

    html! {
        <Form title="Login" {onsubmit}>
            <TextField label="Username" required=true value={(*username).clone()} oninput={oninput_username}/>
            <PasswordField label="Password" value={(*password).clone()} oninput={oninput_password}/>
            <Button name="submit" value="Login" button_type={ButtonType::Primary} />
        </Form>
    }
}

#[function_component(Logout)]
pub fn logout() -> Html {
    let (_, user_dispatch) = use_store::<UserInfo>();

    wasm_bindgen_futures::spawn_local(async move {
        Request::post("/api/logout").send().await.unwrap();
    });

    user_dispatch.reduce_mut(|s| {
        s.username = None;
        s.is_auth = false;
        s.is_admin = false;
    });

    html! {
        <Redirect<Route> to={Route::Login} />
    }
}
