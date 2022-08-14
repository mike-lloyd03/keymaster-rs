use crate::components::form::{Form, PasswordField, TextField};
use reqwasm::http::Request;
use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <Form title="Login" action="login" submit_label="Login">
            <TextField label="Username" />
            <PasswordField label="Password" />
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
