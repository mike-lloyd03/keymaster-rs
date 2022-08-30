use std::vec::Vec;

use crate::components::form::*;
use crate::components::modal::Modal;
use crate::components::notifier::notify_error;
use crate::components::table::{Cell, Row, Table};
use crate::services::form_actions::{ondelete, onload_all, submit_form};
use crate::services::requests::get;
use crate::services::to_option;
use crate::types::Key;

use yew::prelude::*;
use yew_router::hooks::use_history;

use super::auth::CheckAuth;
use super::Route;

#[function_component(NewKey)]
pub fn new_key() -> Html {
    let name = use_state(String::new);
    let description = use_state(String::new);

    let onsubmit = {
        let key = Key {
            name: (*name).clone(),
            description: to_option((*description).clone()),
            active: true,
        };
        let history = use_history().unwrap();
        submit_form("/api/keys".to_string(), key, history, Route::Keys)
    };

    html! {
        <CheckAuth admin=true>
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
                <CancelButton route={Route::Keys} />
            </Form>
        </CheckAuth>
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

    let show_modal = use_state(|| false);

    {
        let key_name = key_name.clone();
        let description = description.clone();
        let active = active.clone();
        let url = format!("/api/keys/{}", &key_name);
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match get::<Key>(url).await {
                        Ok(k) => {
                            description.set(k.description.unwrap_or_default());
                            active.set(k.active);
                        }
                        Err(e) => notify_error(&e.to_string()),
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
            description: to_option((*description).clone()),
            active: *active,
        };
        let history = use_history().unwrap();
        let path = format!("/api/keys/{}", key_name);
        submit_form(path, key, history, Route::Keys)
    };

    let delete_action = {
        let history = use_history().unwrap();
        let path = format!("/api/keys/{}", key_name);
        ondelete(path, history, Route::Keys)
    };

    html! {
        <CheckAuth admin=true>
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
                <DeleteButton
                    value="Delete Key"
                    route={Route::Keys}
                    show_modal={show_modal.clone()}
                />
                {" "}
                <CancelButton route={Route::Keys} />
            </Form>
            <Modal
                title="Delete Key"
                msg="Are you sure you want to delete this key? All assignments which use this key will also be deleted."
                confirm_action={delete_action}
                {show_modal}
            />
        </CheckAuth>
    }
}

#[function_component(KeyTable)]
pub fn key_table() -> Html {
    let keys = use_state(Vec::<Key>::new);

    // Get keys on load
    {
        let keys = keys.clone();
        use_effect_with_deps(
            move |_| {
                onload_all("/api/keys".into(), keys);
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
        <CheckAuth>
            <div class="container text-light my-3">
                <div class="row justify-content-center">
                    <Table title="Keys" button_label="Add Key" button_route={Route::AddKey}>
                    { for rows }
                    </Table>
                </div>
            </div>
        </CheckAuth>
    }
}
