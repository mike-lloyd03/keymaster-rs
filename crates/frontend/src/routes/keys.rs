use crate::components::form::{Button, ButtonType, CheckboxField, Form, TextField};
use crate::components::notifier::Notification;
use crate::components::table::{Cell, Row, Table};
use crate::routes::{oninput, onsubmit, Route};
use web_sys::HtmlInputElement;

use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::history::History;
use yew_router::hooks::use_history;
use yew_router::prelude::Redirect;
use yewdux::prelude::*;
use yewdux_functional::use_store;

#[derive(PartialEq, Default, Deserialize)]
pub struct Key {
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
}

#[function_component(NewKey)]
pub fn new_key() -> Html {
    let name = use_state(|| "".to_string());
    let description = use_state(|| "".to_string());
    let store = use_store::<BasicStore<Notification>>();

    let oninput_name = oninput(name.clone());
    let oninput_desc = oninput(description.clone());

    let oncancel = {
        let history = use_history().unwrap();
        Callback::once(move |_: MouseEvent| history.push(Route::Keys))
    };

    let onsubmit = {
        let history = use_history().unwrap();
        let fields = vec![("name", name.clone()), ("description", description.clone())];
        onsubmit("api/keys", fields, store, history)
    };

    html! {
        <Form title="New Key" action="keys" {onsubmit}>
            <TextField label="Key Name" name="name" required=true value={(*name).clone()} oninput={oninput_name} pattern=r#"[\w\d]{3,}"# />
            <TextField label="Description" value={(*description).clone()} oninput={oninput_desc} />
            <Button name="submit" value="Add Key" button_type={ButtonType::Primary} />
            {" "}
            <Button
                name="cancel"
                value="Cancel"
                button_type={ButtonType::Secondary}
                onclick={oncancel}
                novalidate=true
            />
        </Form>
    }
}

#[derive(PartialEq, Properties)]
pub struct EditKeyProps {
    pub key_name: String,
}

#[function_component(EditKey)]
pub fn edit_key(props: &EditKeyProps) -> Html {
    html! {
        <Form title="Edit Key" action={ format!("keys/{}", props.key_name.clone()) } >
            <TextField label="Description" />
            <CheckboxField label="Active" />
        </Form>
    }
}

#[function_component(KeyTable)]
pub fn key_table() -> Html {
    let keys = use_state(std::vec::Vec::new);
    let response_code = use_state(|| 0);

    {
        let keys = keys.clone();
        let response_code = response_code.clone();
        use_effect_with_deps(
            move |_| {
                let keys = keys.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::get("http://localhost:8080/api/keys")
                        .send()
                        .await
                        .unwrap();
                    response_code.set(resp.status());

                    match resp.json::<Vec<Key>>().await {
                        Ok(resp) => {
                            keys.set(resp);
                        }
                        Err(e) => log::error!("{}", e),
                    }
                });
                || ()
            },
            (),
        );
    }

    let rows = keys.iter().map(|key| {
        html_nested! {
            <Row>
                <Cell heading="Key" value={ key.name.clone() } />
                <Cell heading="Description" value={
                    key.description.clone().unwrap_or_else(|| "".to_string())
                } />
                <Cell heading="Status" value={
                    match key.active {
                        true => "Active",
                        false => "Inactive",
                    }
                } />
                <Cell heading="" value="Edit" />
            </Row>
        }
    });

    if *response_code == 401 {
        return html! {
            <Redirect<Route> to={Route::Login}/>
        };
    }

    html! {
        <div class="container text-light my-3">
            <div class="row justify-content-center">
                <Table title="Keys" button_label="Add Key" button_route={Route::AddKey}>
                { for rows }
                </Table>
            </div>
        </div>
    }
}
