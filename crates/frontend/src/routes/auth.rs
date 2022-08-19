use crate::components::{
    form::{Button, ButtonType, Form, PasswordField, TextField},
    notifier::Notification,
};
use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::use_history;
use yewdux::prelude::BasicStore;
use yewdux_functional::use_store;

use super::{oninput_string, onsubmit, Route};

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(String::new);
    let password = use_state(String::new);

    let oninput_username = oninput_string(username.clone());
    let oninput_password = oninput_string(password.clone());

    let onsubmit = {
        let json = serde_json::json!({
            "username": (*username).clone(),
            "password": (*password).clone(),
        });
        let store = use_store::<BasicStore<Notification>>();
        let history = use_history().unwrap();
        onsubmit("api/login".to_string(), json, store, history, Route::Home)
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
    wasm_bindgen_futures::spawn_local(async move {
        Request::post("http://localhost:8080/api/logout")
            .send()
            .await
            .unwrap();
    });

    html! {
        {"Logged out"}
    }
}
