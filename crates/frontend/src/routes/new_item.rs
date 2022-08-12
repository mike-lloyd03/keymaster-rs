use yew::prelude::*;

use crate::routes::forms::{CheckboxField, Form, TextField};

#[function_component(NewKey)]
pub fn new_key() -> Html {
    html! {
        <div class="container text-light my-3" style="max-width: 600px;">
            <div class="row justify-content-center">
                <Form title="New Key" item="Key">
                    <TextField label="Key Name" />
                    <TextField label="Description" />
                </Form>
            </div>
        </div>
    }
}

#[function_component(NewUser)]
pub fn new_user() -> Html {
    html! {
        <div class="container text-light my-3" style="max-width: 600px;">
            <div class="row justify-content-center">
                <Form title="New User" item="User">
                    <TextField label="Username" />
                    <TextField label="Email" />
                    <TextField label="Display Name" />
                    <CheckboxField label="Can Login?" />
                </Form>
            </div>
        </div>
    }
}
