use crate::components::form::{Form, PasswordField, TextField};
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
