use std::vec::Vec;

use crate::components::details_card::*;
use crate::components::form::*;
use crate::components::modal::Modal;
use crate::components::notifier::notify_error;
use crate::components::table::*;
use crate::services::form_actions::{ondelete, onload, submit_form};
use crate::services::get_display_name;
use crate::services::requests::get;
use crate::services::to_option;
use crate::theme::FORM_SUBTITLE;
use crate::types::Assignment;
use crate::types::Key;
use crate::types::User;

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
            <div class="container my-5 mx-auto">
                <Form title="New Key" {onsubmit}>
                    <TextField
                        label="Key Name"
                        required=true
                        state={name}
                    />
                    <TextField label="Description" state={description} />
                    <Button
                    value="Add Key"
                    button_type={ButtonType::Primary}
                    />
                    {" "}
                    <RouteButton value="Cancel" route={Route::Keys} />
                </Form>
            </div>
        </CheckAuth>
    }
}

#[derive(PartialEq, Eq, Properties)]
pub struct KeyProps {
    pub key_name: String,
}

#[function_component(EditKey)]
pub fn edit_key(props: &KeyProps) -> Html {
    let key_name = use_state(String::new);
    let description = use_state(String::new);
    let active = use_state(|| false);

    let show_modal = use_state(|| false);

    {
        let key_name = key_name.clone();
        let description = description.clone();
        let active = active.clone();
        let url = format!("/api/keys/{}", props.key_name.clone());
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match get::<Key>(url).await {
                        Ok(k) => {
                            // Setting the key name otherwise it will be url
                            // encoded
                            key_name.set(k.name);
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
            name: (*key_name).clone(),
            description: to_option((*description).clone()),
            active: *active,
        };
        let history = use_history().unwrap();
        let path = format!("/api/keys/{}", props.key_name.clone());
        submit_form(path, key, history, Route::Keys)
    };

    let delete_action = {
        let history = use_history().unwrap();
        let path = format!("/api/keys/{}", props.key_name.clone());
        ondelete(path, history, Route::Keys)
    };

    html! {
        <CheckAuth admin=true>
            <div class="container my-5 mx-auto">
                <Form title="Edit Key" {onsubmit}>
                    <h6 class={FORM_SUBTITLE}>
                        {format!("Key: {}", (*key_name).clone())}
                    </h6>
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
                    <RouteButton value="Cancel" route={Route::Keys} />
                </Form>
                <Modal
                    title="Delete Key"
                    msg="Are you sure you want to delete this key? All assignments which use this key will also be deleted."
                    confirm_action={delete_action}
                    {show_modal}
                />
            </div>
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
                onload("/api/keys".into(), keys);
                || ()
            },
            (),
        );
    }

    // Create table rows
    let rows = keys.iter().map(|key| {
        let description = match key.description.clone() {
            Some(d) => {
                if d.is_empty() {
                    "-".to_string()
                } else {
                    d
                }
            }
            None => "-".to_string(),
        };
        let active = match key.active {
            true => "Active",
            false => "Inactive",
        };
        html_nested! {
            <Row>
                <CellLink value={ key.name.clone() } route={Route::KeyDetails { key_name: key.name.clone() }}/>
                <Cell value={description} />
                <Cell value={active} />
            </Row>
        }
    });

    html! {
        <CheckAuth>
            <div class="container my-5 mx-auto max-w-4xl">
                <TableCard
                    title="Keys"
                    headings={vec!["Key", "Description", "Status"]}
                    button_label="Add Key"
                    button_route={Route::AddKey}
                >
                    { for rows }
                </TableCard>
            </div>
        </CheckAuth>
    }
}

#[function_component(KeyDetails)]
pub fn key_details(props: &KeyProps) -> Html {
    let key = use_state(|| Key::default());
    let assignments = use_state(Vec::<Assignment>::new);
    let users = use_state(Vec::<User>::new);

    {
        let key = key.clone();
        let assignments = assignments.clone();
        let users = users.clone();

        let key_url = format!("/api/keys/{}", &props.key_name);
        let key_users_url = format!("/api/assignments?key={}", &props.key_name);

        use_effect_with_deps(
            move |_| {
                onload(key_url, key);
                onload(key_users_url, assignments);
                onload("/api/users".into(), users);
                || ()
            },
            (),
        );
    }

    let key = (*key).clone();
    html! {
        <CheckAuth>
            <DetailsCard
                title={key.name.clone()}
                edit_route={Route::EditKey { key_name: key.name.clone() }}
            >
                <DetailsHeader>
                    <DetailsHeaderItem content={format!("Description: {}", key.description.unwrap_or("-".into()))} />
                    <DetailsHeaderItem content={format!("Active: {}", key.active)} />
                </DetailsHeader>
                <DetailsList label="Assigned Users">
                    { for (*assignments)
                        .iter()
                            .map(|a|
                                html_nested!{
                                    <DetailsListItem
                                        label={get_display_name(&(*users).clone(), a.clone().user)}
                                        route={Route::AssignmentDetails { id: a.clone().id } }
                                    />
                                })
                    }
                </DetailsList>
                <DetailsFooter/>
            </DetailsCard>
        </CheckAuth>
    }
}
