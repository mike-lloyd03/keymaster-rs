use std::vec::Vec;

use crate::components::form::{Button, ButtonType, CheckboxField, Form, TextField};
use crate::components::notifier::notify;
use crate::components::table::{Cell, Row, Table};
use crate::error::Error;
use crate::services::form_actions::{ondelete, onload_all, onsubmit};
use crate::services::handle_unauthorized;
use crate::services::requests::get;
use crate::types::{Key, Notification};

use yew::prelude::*;
use yew_router::history::History;
use yew_router::hooks::use_history;
use yewdux::prelude::*;

use super::Route;

#[function_component(NewKey)]
pub fn new_key() -> Html {
    let name = use_state(String::new);
    let description = use_state(String::new);

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
        let (_, dispatch) = use_store::<Notification>();
        let history = use_history().unwrap();
        onsubmit("/api/keys".to_string(), key, dispatch, history, Route::Keys)
    };

    html! {
        <Form title="New Key" {onsubmit}>
            <TextField
                label="Key Name"
                required=true
                state={name}
                pattern=r#"[\w\d]{3,}"#
            />
            <TextField label="Description" state={description} />
            <Button
                value="Add Key"
                button_type={ButtonType::Primary}
            />
            {" "}
            <Button
                value="Cancel"
                button_type={ButtonType::Secondary}
                onclick={oncancel}
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

    {
        let key_name = key_name.clone();
        let description = description.clone();
        let active = active.clone();
        let (_, dispatch) = use_store::<Notification>();
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
                            Error::Unauthorized => handle_unauthorized(history, dispatch),
                            _ => notify(&dispatch, e.to_string(), "error".into()),
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
            active: *active,
        };
        let (_, dispatch) = use_store::<Notification>();
        let history = use_history().unwrap();
        let path = format!("/api/keys/{}", key_name);
        onsubmit(path, key, dispatch, history, Route::Keys)
    };

    let ondelete = {
        let (_, dispatch) = use_store::<Notification>();
        let history = use_history().unwrap();
        let path = format!("/api/keys/{}", key_name);
        ondelete(path, dispatch, history, Route::Keys)
    };

    let oncancel = {
        let history = use_history().unwrap();
        Callback::once(move |_: MouseEvent| history.push(Route::Keys))
    };

    html! {
        <>
        <Form title="Edit Key"
            subtitle={props.key_name.clone()}
            action={format!("keys/{}", props.key_name.clone())}
            {onsubmit}
        >
            <TextField label="Description" state={description} />
            <CheckboxField label="Active" state={active} />
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
            />
        </Form>
        </>
    }
}

#[function_component(KeyTable)]
pub fn key_table() -> Html {
    let keys = use_state(Vec::<Key>::new);

    // Get keys on load
    {
        let (_, dispatch) = use_store::<Notification>();
        let history = use_history().unwrap();
        let keys = keys.clone();
        use_effect_with_deps(
            move |_| {
                onload_all("/api/keys".into(), dispatch, history, keys);
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
