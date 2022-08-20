use crate::components::{
    form::{Button, ButtonType, Form, PasswordField, TextField},
    notifier::Notification,
    user_context_provider::UserInfo,
};
use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
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
        onsubmit("api/login".into(), json, store, history, Route::Home)
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
    let user_store = use_store::<BasicStore<UserInfo>>();

    wasm_bindgen_futures::spawn_local(async move {
        Request::post("/api/logout").send().await.unwrap();
    });

    user_store.dispatch().reduce(|s| {
        s.username = None;
        s.is_auth = false;
        s.is_admin = false;
    });

    html! {
        <Redirect<Route> to={Route::Login} />
    }
}
