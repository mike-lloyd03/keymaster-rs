use std::vec::Vec;

use crate::components::form::{
    Button, ButtonType, DateField, Form, MultiSelectField, MultiSelectOption, TextField,
};
use crate::components::notifier::notify;
use crate::components::table::{Cell, Row, Table};
use crate::error::Error;
use crate::services::form_actions::{ondelete, onload_all, onsubmit};
use crate::services::requests::get;
use crate::services::{format_date, handle_unauthorized, parse_date};
use crate::types::{Assignment, Notification};

use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

use super::Route;

#[function_component(NewAssignment)]
pub fn new_assignment() -> Html {
    let available_users = use_state(Vec::new);
    let available_keys = use_state(Vec::new);
    let date_out = use_state(String::new);
    let selected_users = use_state(Vec::<String>::new);
    let selected_keys = use_state(Vec::<String>::new);

    {
        let users = available_users.clone();
        let keys = available_keys.clone();
        let (_, dispatch) = use_store::<Notification>();
        use_effect_with_deps(
            move |_| {
                crate::services::form_actions::get_options(users, keys, dispatch);
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
        let date_out = parse_date((*date_out).clone());

        for user in &*selected_users {
            for key in &*selected_keys {
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
            <MultiSelectField label="User" state={selected_users}>
                { for user_options.clone() }
            </MultiSelectField>
            <MultiSelectField label="Key" state={selected_keys}>
                { for key_options }
            </MultiSelectField>
            <DateField
                label="Date Out"
                required=true
                state={date_out}
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
            />
        </Form>
    }
}

#[derive(PartialEq, Eq, Properties)]
pub struct EditAssignmentProps {
    pub id: i64,
}

#[function_component(EditAssignment)]
pub fn edit_assignment(props: &EditAssignmentProps) -> Html {
    let user = use_state(String::new);
    let key = use_state(String::new);
    let date_out = use_state(String::new);
    let date_in = use_state(String::new);

    {
        let user = user.clone();
        let key = key.clone();
        let date_out = date_out.clone();
        let date_in = date_in.clone();
        let (_, dispatch) = use_store::<Notification>();
        let history = use_history().unwrap();
        let url = format!("/api/assignments/{}", &props.id.clone());
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match get::<Assignment>(url).await {
                        Ok(a) => {
                            user.set(a.user);
                            key.set(a.key);
                            date_out.set(format_date(a.date_out));
                            date_in.set(match a.date_in {
                                Some(d) => format_date(d),
                                None => "".into(),
                            });
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
        let assignment = Assignment {
            user: (*user).clone(),
            key: (*key).clone(),
            date_out: parse_date((*date_out).clone()),
            date_in: Some(parse_date((*date_in).clone())),
            ..Default::default()
        };
        let (_, dispatch) = use_store::<Notification>();
        let history = use_history().unwrap();
        let path = format!("/api/assignments/{}", props.id);
        onsubmit(path, assignment, dispatch, history, Route::Assignments)
    };

    let ondelete = {
        let (_, dispatch) = use_store::<Notification>();
        let history = use_history().unwrap();
        let path = format!("/api/assignments/{}", props.id);
        ondelete(path, dispatch, history, Route::Assignments)
    };

    let oncancel = {
        let history = use_history().unwrap();
        Callback::once(move |_: MouseEvent| history.push(Route::Assignments))
    };

    html! {
        <Form title="Edit Assignment" {onsubmit}>
            <TextField label="User" state={user}/>
            <TextField label="Key" state={key}/>
            <DateField label="Date Out" state={date_out}/>
            <DateField label="Date In" state={date_in}/>
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
    }
}
#[function_component(Assignments)]
pub fn assignments() -> Html {
    let assignments = use_state(Vec::<Assignment>::new);

    // Get assignments on load
    {
        let (_, dispatch) = use_store::<Notification>();
        let history = use_history().unwrap();
        let assignments = assignments.clone();
        use_effect_with_deps(
            move |_| {
                onload_all("/api/assignments".into(), dispatch, history, assignments);
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
