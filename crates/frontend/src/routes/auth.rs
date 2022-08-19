use crate::components::form::{Button, ButtonType, Form, PasswordField, TextField};
use gloo_net::http::Request;
use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <Form title="Login" action="login" >
            <TextField label="Username" />
            <PasswordField label="Password" />
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
