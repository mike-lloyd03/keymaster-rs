use crate::services::auth::login_user;
use crate::types::Credentials;
use crate::{
    components::form::{Button, ButtonType, Form, PasswordField, TextField},
    services::auth::logout_user,
};
use yew::prelude::*;
use yew_router::prelude::*;

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
            let creds = creds.clone();
            let history = history.clone();
            e.prevent_default();
            wasm_bindgen_futures::spawn_local(async move {
                if (login_user(creds, &history).await).is_ok() {
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
    wasm_bindgen_futures::spawn_local(async move {
        logout_user().await;
    });

    html! {
        <Redirect<Route> to={Route::Login} />
    }
}
