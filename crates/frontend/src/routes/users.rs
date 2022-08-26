use std::vec::Vec;

use crate::components::form::{Button, ButtonType, CheckboxField, Form, TextField};
use crate::components::modal::Modal;
use crate::components::notifier::notify_error;
use crate::components::table::{Cell, Row, Table};
use crate::error::Error;
use crate::services::form_actions::{ondelete, onload_all, submit_form};
use crate::services::handle_unauthorized;
use crate::services::requests::get;
use crate::types::User;
use yew::prelude::*;
use yew_router::prelude::*;

use super::Route;

#[function_component(NewUser)]
pub fn new_user() -> Html {
    let username = use_state(String::new);
    let email = use_state(String::new);
    let display_name = use_state(String::new);
    let can_login = use_state(|| false);

    let oncancel = {
        let history = use_history().unwrap();
        Callback::once(move |_: MouseEvent| history.push(Route::Users))
    };

    let onsubmit = {
        let user = User {
            username: (*username).clone(),
            email: if email.is_empty() {
                None
            } else {
                Some((*email).clone())
            },
            display_name: if display_name.is_empty() {
                None
            } else {
                Some((*display_name).clone())
            },
            can_login: false,
            ..Default::default()
        };
        let history = use_history().unwrap();
        submit_form("/api/users".to_string(), user, history, Route::Users)
    };

    html! {
        <Form title="New User" {onsubmit}>
            <TextField
                label="Username"
                required=true
                state={username}
                pattern=r#"[\w\d]{3,}"#
            />
            <TextField
                label="Email"
                state={email}
                pattern=r#"[^@]+@[\w\d]+\.[\w\.]{2,}"#
            />
            <TextField
                label="Display Name"
                state={display_name}
            />
            <CheckboxField label="Can Login?" state={can_login} />
            <Button
                value="Add User"
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
pub struct EditUserProps {
    pub username: String,
}

#[function_component(EditUser)]
pub fn edit_user(props: &EditUserProps) -> Html {
    let id = use_state(|| 0);
    let username = props.username.clone();
    let email = use_state(String::new);
    let display_name = use_state(String::new);
    let can_login = use_state(|| false);
    let admin = use_state(|| false);

    let show_confirm_modal = use_state(|| false);

    {
        let id = id.clone();
        let username = username.clone();
        let email = email.clone();
        let display_name = display_name.clone();
        let can_login = can_login.clone();
        let admin = admin.clone();
        let history = use_history().unwrap();
        let url = format!("/api/users/{}", &username);
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match get::<User>(url).await {
                        Ok(u) => {
                            id.set(u.id);
                            email.set(u.email.unwrap_or_default());
                            display_name.set(u.display_name.unwrap_or_default());
                            can_login.set(u.can_login);
                            admin.set(u.admin);
                        }
                        Err(e) => match e {
                            Error::Unauthorized => handle_unauthorized(history),
                            _ => notify_error(&e.to_string()),
                        },
                    }
                });
                || ()
            },
            (),
        );
    }

    let onsubmit = {
        let user = User {
            id: (*id),
            username: username.clone(),
            email: Some((*email).clone()),
            display_name: Some((*display_name).clone()),
            can_login: *can_login,
            admin: *admin,
        };
        let history = use_history().unwrap();
        let path = format!("/api/users/{}", username);
        submit_form(path, user, history, Route::Users)
    };

    let onclick_delete = {
        let show_confirm_modal = show_confirm_modal.clone();
        Callback::once(move |e: MouseEvent| {
            e.prevent_default();
            show_confirm_modal.set(true);
        })
    };

    let delete_action = {
        let history = use_history().unwrap();
        let path = format!("/api/users/{}", username);
        ondelete(path, history, Route::Users)
    };

    let oncancel = {
        let history = use_history().unwrap();
        Callback::once(move |_: MouseEvent| history.push(Route::Users))
    };

    html! {
        <Form title="Edit User" subtitle={props.username.clone()} action={format!("users/{}", props.username.clone())} {onsubmit} >
            <TextField
                label="Email"
                state={email}
                pattern=r#"[^@]+@[\w\d]+\.[\w\.]{2,}"#
            />
            <TextField
                label="Display Name"
                state={display_name}
            />
            <CheckboxField label="Can Login?" state={can_login} />
            <Button
                value="Update User"
                button_type={ButtonType::Primary}
            />
            {" "}
            <Button
                value="Delete User"
                button_type={ButtonType::Danger}
                onclick={onclick_delete}
            />
            {" "}
            <Button
                value="Cancel"
                button_type={ButtonType::Secondary}
                onclick={oncancel}
            />
        <Modal
            title="Delete User"
            msg="Are you sure you want to delete this user? All assignments for with this user is assigned will also be deleted."
            confirm_action={delete_action}
            show_modal={show_confirm_modal}
        />
        </Form>
    }
}

#[function_component(UserTable)]
pub fn user_table() -> Html {
    let users = use_state(Vec::<User>::new);

    // Get users on load
    {
        let history = use_history().unwrap();
        let users = users.clone();
        use_effect_with_deps(
            move |_| {
                onload_all("/api/users".into(), history, users);
                || ()
            },
            (),
        );
    }

    // Create table rows
    let rows = users.iter().map(|user| {
        let username = user
            .display_name
            .clone()
            .unwrap_or_else(|| user.username.clone());
        let email = user.email.clone();
        html_nested! {
            <Row>
                <Cell heading="User" value={username} />
                <Cell heading="Email" value={email} />
                <Cell heading="" edit_route={Route::EditUser {username: user.username.clone()}} />
            </Row>
        }
    });

    html! {
        <div class="container text-light my-3">
            <div class="row justify-content-center">
                <Table title="Users" button_label="Add User" button_route={Route::AddUser}>
                { for rows }
                </Table>
            </div>
        </div>
    }
}
