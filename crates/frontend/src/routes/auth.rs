use crate::services::auth::login_user;
use crate::types::{Credentials, Notification, UserInfo};
use crate::{
    components::form::{Button, ButtonType, Form, PasswordField, TextField},
    services::auth::logout_user,
};
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
                if (login_user(creds, &user_dispatch, &notify_dispatch, &history).await).is_ok() {
                    history.push(Route::Home)
                }
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
        logout_user(&user_dispatch).await;
    });

    html! {
        <Redirect<Route> to={Route::Login} />
    }
}
