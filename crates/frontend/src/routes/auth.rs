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

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(String::new);
    let password = use_state(String::new);

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
            <TextField label="Username" required=true state={username}/>
            <PasswordField label="Password" state={password}/>
            <Button value="Login" button_type={ButtonType::Primary} />
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
