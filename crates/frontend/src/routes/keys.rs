use super::{ondelete, oninput_bool, oninput_string, onsubmit, Route};
use crate::components::form::{Button, ButtonType, CheckboxField, Form, TextField};
use crate::components::notifier::{notify, Notification};
use crate::components::table::{Cell, Row, Table};

use gloo_net::http::Request;
use serde::Deserialize;
use serde_json::json;
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
    let name = use_state(String::new);
    let description = use_state(String::new);

    let oninput_name = oninput_string(name.clone());
    let oninput_desc = oninput_string(description.clone());

    let oncancel = {
        let history = use_history().unwrap();
        Callback::once(move |_: MouseEvent| history.push(Route::Keys))
    };

    let onsubmit = {
        let json = json!({
            "name": (*name).clone(),
            "description": (*description).clone()
        });
        let store = use_store::<BasicStore<Notification>>();
        let history = use_history().unwrap();
        onsubmit("/api/keys".to_string(), json, store, history, Route::Keys)
    };

    html! {
        <Form title="New Key" {onsubmit}>
            <TextField label="Key Name" required=true value={(*name).clone()} oninput={oninput_name} pattern=r#"[\w\d]{3,}"# />
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
    let description = use_state(String::new);
    let active = use_state(|| false);
    let key_name = props.key_name.clone();

    {
        let description = description.clone();
        let active = active.clone();
        let store = use_store::<BasicStore<Notification>>();
        let history = use_history().unwrap();
        let key_name = key_name.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::get(&format!("/api/keys/{}", &key_name))
                        .send()
                        .await
                        .unwrap();

                    if resp.ok() {
                        match resp.json::<Key>().await {
                            Ok(k) => {
                                description.set(k.description.unwrap_or_default());
                                active.set(k.active);
                            }
                            Err(e) => log::error!("{}", e),
                        }
                    } else if resp.status() == 404 {
                        history.push(Route::NotFound);
                    } else {
                        let resp_text = resp.text().await;
                        notify(store, resp_text.unwrap(), "error".to_string());
                    }
                });
                || ()
            },
            (),
        );
    }

    let oninput_desc = oninput_string(description.clone());
    let oninput_active = oninput_bool(active.clone());

    let oncancel = {
        let history = use_history().unwrap();
        Callback::once(move |_: MouseEvent| history.push(Route::Keys))
    };

    let ondelete = {
        let store = use_store::<BasicStore<Notification>>();
        let history = use_history().unwrap();
        let path = format!("/api/keys/{}", key_name);
        ondelete(path, store, history, Route::Keys)
    };

    let onsubmit = {
        let json = json!({
            "description": (*description).clone(),
            "active": (*active).clone(),
        });
        let store = use_store::<BasicStore<Notification>>();
        let history = use_history().unwrap();
        let path = format!("/api/keys/{}", key_name);
        onsubmit(path, json, store, history, Route::Keys)
    };

    html! {
        <>
        <Form title="Edit Key" action={format!("keys/{}", props.key_name.clone())} {onsubmit} >
            <TextField label="Description" value={(*description).clone()} oninput={oninput_desc} />
            <CheckboxField label="Active" checked={*active} onchange={oninput_active} />
            <Button name="submit" value="Update Key" button_type={ButtonType::Primary} />
            {" "}
            <Button
                name="delete"
                value="Delete Key"
                button_type={ButtonType::Danger}
                onclick={ondelete}
            />
            {" "}
            <Button
                name="cancel"
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
                    key.description.clone().unwrap_or_default()
                } />
                <Cell heading="Status" value={
                    match key.active {
                        true => "Active",
                        false => "Inactive",
                    }
                } />
                <Cell heading="" edit_route={Route::EditKey {key_name: key.name.clone()}} />
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
