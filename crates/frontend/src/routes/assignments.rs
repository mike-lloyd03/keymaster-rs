use crate::components::form::{
    Button, ButtonType, DateField, Form, MultiSelectField, MultiSelectOption, TextField,
};
use crate::components::notifier::notify;
use crate::components::table::{Cell, Row, Table};
use crate::error::Error;
use crate::services::form_actions::{oninput_option, oninput_select, oninput_string, onsubmit};
use crate::services::requests::{self, get};
use crate::types::{Assignment, Key, Notification, User};
use chrono::NaiveDate;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

use super::Route;

#[function_component(NewAssignment)]
pub fn new_assignment() -> Html {
    let available_users = use_state(std::vec::Vec::new);
    let available_keys = use_state(std::vec::Vec::new);
    let date_out = use_state(String::new);
    let selected_users = use_state(std::vec::Vec::new);
    let selected_keys = use_state(std::vec::Vec::new);

    let oninput_users = oninput_select(selected_users.clone());
    let oninput_keys = oninput_select(selected_keys.clone());
    let oninput_date_out = oninput_string(date_out.clone());

    {
        let users = available_users.clone();
        let keys = available_keys.clone();
        let (_, dispatch) = use_store::<Notification>();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match get::<Vec<User>>("/api/users".into()).await {
                        Ok(u) => users.set(u.iter().map(|user| user.username.clone()).collect()),
                        Err(e) => notify(&dispatch, e.to_string(), "error".into()),
                    };
                    match get::<Vec<Key>>("/api/keys?active=true".into()).await {
                        Ok(k) => keys.set(k.iter().map(|key| key.name.clone()).collect()),
                        Err(e) => notify(&dispatch, e.to_string(), "error".into()),
                    }
                });
                || ()
            },
            (),
        );
    }

    let oncancel = {
        let history = use_history().unwrap();
        Callback::once(move |_: MouseEvent| history.push(Route::Assignments))
    };

    let onsubmit = {
        let mut assignments: Vec<Assignment> = Vec::new();
        let users = (*selected_users).clone();
        let keys = (*selected_keys).clone();
        let date_out =
            NaiveDate::parse_from_str(&(*date_out).clone(), "%Y-%m-%d").unwrap_or_default();
        for user in &users {
            for key in &keys {
                let a = Assignment {
                    user: user.into(),
                    key: key.into(),
                    date_out,
                    ..Default::default()
                };
                assignments.push(a);
            }
        }
        let (_, dispatch) = use_store::<Notification>();
        let history = use_history().unwrap();
        onsubmit(
            "/api/assignments".to_string(),
            assignments,
            dispatch,
            history,
            Route::Assignments,
        )
    };

    let user_options = available_users.iter().map(|user| {
        html_nested! {
            <MultiSelectOption value={ user.clone() } label={ user.clone() } />
        }
    });

    let key_options = available_keys.iter().map(|key| {
        html_nested! {
            <MultiSelectOption value={key.to_string()} />
        }
    });

    html! {
        <Form title="Assign Key" {onsubmit}>
            <MultiSelectField label="User" onchange={oninput_users}>
                { for user_options.clone() }
            </MultiSelectField>
            <MultiSelectField label="Key" onchange={oninput_keys}>
                { for key_options }
            </MultiSelectField>
            <DateField
                label="Date Out"
                required=true
                value={(*date_out).clone()}
                oninput={oninput_date_out}
            />
            <Button
                value="Assign Key"
                button_type={ButtonType::Primary}
            />
            {" "}
            <Button
                value="Cancel"
                button_type={ButtonType::Secondary}
                onclick={oncancel}
                novalidate=true
            />
            <p>{(*selected_users).clone()}</p>
            <p>{(*selected_keys).clone()}</p>
            <p>{(*date_out).clone()}</p>
        </Form>
    }
}

#[derive(PartialEq, Eq, Properties)]
pub struct EditAssignmentProps {
    pub id: i64,
}

#[function_component(EditAssignment)]
pub fn edit_assignment(props: &EditAssignmentProps) -> Html {
    html! {
        <Form title="Edit Assignment">
            <TextField label="User" />
            <TextField label="Key" />
            <DateField label="Date Out" />
            <DateField label="Date In" />
        </Form>
    }
}
#[function_component(Assignments)]
pub fn assignments() -> Html {
    let assignments = use_state(std::vec::Vec::new);

    // Get assignments on load
    {
        let (_, dispatch) = use_store::<Notification>();
        let history = use_history().unwrap();
        let assignments = assignments.clone();
        use_effect_with_deps(
            move |_| {
                let assignments = assignments.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match get::<Vec<Assignment>>("/api/assignments".into()).await {
                        Ok(a) => assignments.set(a),
                        Err(e) => match e {
                            Error::Unauthorized => {
                                history.push(Route::Login);
                                notify(
                                    &dispatch,
                                    "You must log in to access this page".into(),
                                    "error".into(),
                                );
                            }
                            _ => notify(&dispatch, e.to_string(), "error".into()),
                        },
                    }
                });
                || ()
            },
            (),
        );
    }

    let rows = assignments.iter().map(|a| {
        html_nested! {
            <Row>
                <Cell heading="User" value={
                    a.user.clone()
                } />
                <Cell heading="Key" value={
                    a.key.clone()
                } />
                <Cell heading="Date Out" value={
                    a.date_out.format("%Y-%m-%d").to_string()
                } />
                <Cell heading="Date In" value={
                    match a.date_in {
                        Some(d) => d.format("%Y-%m-%d").to_string(),
                        None => "".to_string(),
                    }
                } />
                <Cell heading="" edit_route={Route::EditAssignment {id: a.id.clone()}} />
            </Row>
        }
    });

    html! {
        <div class="container text-light my-3">
            <div class="row justify-content-center">
                <Table title="Assignments" button_label="Assign Key" button_route={Route::AssignKey}>
                { for rows }
                </Table>
            </div>
        </div>
    }
}
