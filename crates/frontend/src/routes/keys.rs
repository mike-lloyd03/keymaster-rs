use crate::components::form::{CheckboxField, Form, TextField};
use crate::components::table::{Cell, Row, Table};
use crate::routes::Route;
use web_sys::HtmlInputElement;

// use reqwasm::http::Request;
use gloo_net::http::Request;
use serde::Deserialize;
use serde_json::json;
use yew::prelude::*;
use yew_router::history::History;
use yew_router::hooks::use_history;
use yew_router::prelude::Redirect;

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

    fn onchange(state: UseStateHandle<String>) -> Callback<Event> {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            state.set(input.value());
        })
    }

    let onchange_name = onchange(name.clone());
    let onchange_desc = onchange(description.clone());

    let onsubmit = {
        let history = use_history().unwrap();
        let name = name.clone();
        let description = description.clone();
        Callback::once(move |e: FocusEvent| {
            e.prevent_default();
            wasm_bindgen_futures::spawn_local(async move {
                Request::post("http://localhost:8080/api/keys")
                    // .header("content-type", "application/json")
                    .json(&json!({
                        "name": (*name).clone(),
                        "description": (*description).clone()
                    }))
                    .unwrap()
                    .send()
                    .await
                    .unwrap();
            });
            history.push(Route::Keys)
        })
    };

    html! {
        <Form title="New Key" action="keys" submit_label="Add Key" {onsubmit}>
            <TextField label="Key Name" name="name" value={(*name).clone()} onchange={onchange_name} />
            <TextField label="Description" value={(*description).clone()} onchange={onchange_desc} />
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
        <Form title="Edit Key" action={ format!("keys/{}", props.key_name.clone()) } submit_label="Save Changes">
            <TextField label="Description" />
            <CheckboxField label="Active" />
        </Form>
    }
}

#[function_component(KeyTable)]
pub fn key_table() -> Html {
    let keys = use_state(|| vec![]);
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
                <Table title="Keys">
                { for rows }
                </Table>
            </div>
        </div>
    }
}
