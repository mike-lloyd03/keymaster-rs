use crate::components::form::{CheckboxField, Form, TextField};
use crate::components::table::{Cell, Row, Table};
use yew::prelude::*;

#[derive(PartialEq, Default)]
pub struct Key {
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
}

#[function_component(NewKey)]
pub fn new_key() -> Html {
    html! {
        <Form title="New Key" action_label="Key">
            <TextField label="Key Name" />
            <TextField label="Description" />
        </Form>
    }
}

#[function_component(EditKey)]
pub fn edit_key() -> Html {
    html! {
        <Form title="Edit Key" action_label="Save Changes">
            <TextField label="Description" />
            <CheckboxField label="Active" />
        </Form>
    }
}

#[function_component(KeyTable)]
pub fn key_table() -> Html {
    let keys = vec![
        Key {
            name: "key1".to_string(),
            description: Some("this is key 1".to_string()),
            active: true,
        },
        Key {
            name: "key2".to_string(),
            description: Some("this is key 2".to_string()),
            active: true,
        },
        Key {
            name: "key4".to_string(),
            active: false,
            ..Default::default()
        },
    ];

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
                <Table title="Keys">
                { for rows }
                </Table>
            </div>
        </div>
    }
}
