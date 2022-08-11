use yew::prelude::*;

use crate::routes::forms::Form;

#[function_component(NewKey)]
pub fn new_key() -> Html {
    html! {
        <div class="container text-light my-3" style="max-width: 600px;">
            <div class="row justify-content-center">
                <Form item="Key" fields={vec!["name", "description"]} />
            </div>
        </div>
    }
}
