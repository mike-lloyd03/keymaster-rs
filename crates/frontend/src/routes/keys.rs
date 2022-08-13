use crate::components::form::{CheckboxField, Form, TextField};
use crate::components::table::{Cell, Row, Table};
use reqwasm::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(PartialEq, Default, Deserialize)]
pub struct Key {
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
}

#[function_component(NewKey)]
pub fn new_key() -> Html {
    html! {
        <Form title="New Key" action="keys" submit_label="Add Key">
            <TextField label="Key Name" name="name" />
            <TextField label="Description" />
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
    {
        let keys = keys.clone();
        use_effect_with_deps(
            move |_| {
                let keys = keys.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_keys: Vec<Key> = Request::get("http://localhost:8081/keys")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    keys.set(fetched_keys);
                });
                || ()
            },
            (),
        );
    }
    // let keys = vec![
    //     Key {
    //         name: "key1".to_string(),
    //         description: Some("this is key 1".to_string()),
    //         active: true,
    //     },
    //     Key {
    //         name: "key2".to_string(),
    //         description: Some("this is key 2".to_string()),
    //         active: true,
    //     },
    //     Key {
    //         name: "key4".to_string(),
    //         active: false,
    //         ..Default::default()
    //     },
    // ];

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

    html! {
        <div class="container text-light my-3">
            <div class="row justify-content-center">
                { format!("keys retrieved: {}", keys.len())}
                <Table title="Keys">
                { for rows }
                </Table>
            </div>
        </div>
    }
}
