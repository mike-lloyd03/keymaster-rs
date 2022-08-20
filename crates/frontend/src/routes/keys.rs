use crate::components::form::{Button, ButtonType, CheckboxField, Form, TextField};
use crate::components::notifier::{notify, Notification};
use crate::components::table::{Cell, Row, Table};
use crate::error::Error;
use crate::services::form_actions::{ondelete, oninput_bool, oninput_string, onsubmit};
use crate::services::requests::get;

use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::history::History;
use yew_router::hooks::use_history;
use yewdux::prelude::*;
use yewdux_functional::use_store;

use super::Route;

#[derive(PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Key {
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
}

#[function_component(NewKey)]
pub fn new_key() -> Html {
    let name = use_state(String::new);
    let description = use_state(String::new);
    let oninput_name = oninput_string(name.clone());
    let oninput_desc = oninput_string(description.clone());

    let oncancel = {
        let history = use_history().unwrap();
        Callback::once(move |_: MouseEvent| history.push(Route::Keys))
    };

    let onsubmit = {
        let key = Key {
            name: (*name).clone(),
            description: Some((*description).clone()),
            active: true,
        };
        let store = use_store::<BasicStore<Notification>>();
        let history = use_history().unwrap();
        onsubmit("/api/keys".to_string(), key, store, history, Route::Keys)
    };

    html! {
        <Form title="New Key" {onsubmit}>
            <TextField
                label="Key Name"
                required=true
                value={(*name).clone()}
                oninput={oninput_name}
                pattern=r#"[\w\d]{3,}"#
            />
            <TextField label="Description" value={(*description).clone()} oninput={oninput_desc} />
            <Button
                value="Add Key"
                button_type={ButtonType::Primary}
            />
            {" "}
            <Button
                value="Cancel"
                button_type={ButtonType::Secondary}
                onclick={oncancel}
                novalidate=true
            />
        </Form>
    }
}

#[derive(PartialEq, Eq, Properties)]
pub struct EditKeyProps {
    pub key_name: String,
}

#[function_component(EditKey)]
pub fn edit_key(props: &EditKeyProps) -> Html {
    let key_name = props.key_name.clone();
    let description = use_state(String::new);
    let active = use_state(|| false);
    let oninput_desc = oninput_string(description.clone());
    let oninput_active = oninput_bool(active.clone());

    {
        let key_name = key_name.clone();
        let description = description.clone();
        let active = active.clone();
        let store = use_store::<BasicStore<Notification>>();
        let history = use_history().unwrap();
        let url = format!("/api/keys/{}", &key_name);
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match get::<Key>(url).await {
                        Ok(k) => {
                            description.set(k.description.unwrap_or_default());
                            active.set(k.active);
                        }
                        Err(e) => match e {
                            Error::Unauthorized => {
                                history.push(Route::Login);
                                notify(
                                    store,
                                    "You must log in to access this page".into(),
                                    "error".into(),
                                );
                            }
                            _ => notify(store, e.to_string(), "error".into()),
                        },
                    }
                });
                || ()
            },
            (),
        );
    }

    let onsubmit = {
        let key = Key {
            name: key_name.clone(),
            description: Some((*description).clone()),
            active: true,
        };
        let store = use_store::<BasicStore<Notification>>();
        let history = use_history().unwrap();
        let path = format!("/api/keys/{}", key_name);
        onsubmit(path, key, store, history, Route::Keys)
    };

    let ondelete = {
        let store = use_store::<BasicStore<Notification>>();
        let history = use_history().unwrap();
        let path = format!("/api/keys/{}", key_name);
        ondelete(path, store, history, Route::Keys)
    };

    let oncancel = {
        let history = use_history().unwrap();
        Callback::once(move |_: MouseEvent| history.push(Route::Keys))
    };

    html! {
        <>
        <Form title="Edit Key" subtitle={props.key_name.clone()} action={format!("keys/{}", props.key_name.clone())} {onsubmit} >
            <TextField label="Description" value={(*description).clone()} oninput={oninput_desc} />
            <CheckboxField label="Active" checked={*active} onchange={oninput_active} />
            <Button
                value="Update Key"
                button_type={ButtonType::Primary}
            />
            {" "}
            <Button
                value="Delete Key"
                button_type={ButtonType::Danger}
                onclick={ondelete}
            />
            {" "}
            <Button
                value="Cancel"
                button_type={ButtonType::Secondary}
                onclick={oncancel}
                novalidate=true
            />
        </Form>
        <p>{*active}</p>
        </>
    }
}

#[function_component(KeyTable)]
pub fn key_table() -> Html {
    let keys = use_state(std::vec::Vec::new);

    // Get keys on load
    {
        let store = use_store::<BasicStore<Notification>>();
        let history = use_history().unwrap();
        let keys = keys.clone();
        use_effect_with_deps(
            move |_| {
                let keys = keys.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match get::<Vec<Key>>("/api/keys".into()).await {
                        Ok(k) => keys.set(k),
                        Err(e) => match e {
                            Error::Unauthorized => {
                                history.push(Route::Login);
                                notify(
                                    store,
                                    "You must log in to access this page".into(),
                                    "error".into(),
                                );
                            }
                            _ => notify(store, e.to_string(), "error".into()),
                        },
                    }
                });
                || ()
            },
            (),
        );
    }

    // Create table rows
    let rows = keys.iter().map(|key| {
        let description = key.description.clone().unwrap_or_default();
        let active = match key.active {
            true => "Active",
            false => "Inactive",
        };
        html_nested! {
            <Row>
                <Cell heading="Key" value={ key.name.clone() } />
                <Cell heading="Description" value={description} />
                <Cell heading="Status" value={active} />
                <Cell heading="" edit_route={Route::EditKey {key_name: key.name.clone()}} />
            </Row>
        }
    });

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
