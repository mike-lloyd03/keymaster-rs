use std::vec::Vec;

use crate::components::details_card::*;
use crate::components::form::*;
use crate::components::modal::Modal;
use crate::components::notifier::notify_error;
use crate::components::table::{Cell, CellLink, Row, TableCard};
use crate::services::form_actions::{ondelete, onload, submit_form};
use crate::services::requests::get;
use crate::services::to_option;
use crate::theme::FORM_SUBTITLE;
use crate::types::Assignment;
use crate::types::{SetPasswdPayload, User};

use yew::prelude::*;
use yew_router::prelude::*;

use super::auth::CheckAuth;
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
            email: to_option((*email).clone()),
            display_name: to_option((*display_name).clone()),
            can_login: (*can_login),
            ..Default::default()
        };
        let history = use_history().unwrap();
        submit_form("/api/users".to_string(), user, history, Route::Users)
    };

    html! {
        <CheckAuth admin=true>
            <div class="container my-5 mx-auto">
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
                        // https://owasp.org/www-community/OWASP_Validation_Regex_Repository
                        pattern=r#"[a-zA-Z0-9_+&*-]+(?:\.[a-zA-Z0-9_+&*-]+)*@(?:[a-zA-Z0-9-]+\.)+[a-zA-Z]{2,15}"#
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
            </div>
        </CheckAuth>
    }
}

#[derive(PartialEq, Eq, Properties)]
pub struct UserProps {
    pub username: String,
}

#[function_component(EditUser)]
pub fn edit_user(props: &UserProps) -> Html {
    let id = use_state(|| 0);
    let username = props.username.clone();
    let email = use_state(String::new);
    let display_name = use_state(String::new);
    let can_login = use_state(|| false);
    let admin = use_state(|| false);

    let show_modal = use_state(|| false);

    // Get user on load
    {
        let id = id.clone();
        let username = username.clone();
        let email = email.clone();
        let display_name = display_name.clone();
        let can_login = can_login.clone();
        let admin = admin.clone();
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
                        Err(e) => notify_error(&e.to_string()),
                    }
                });
                || ()
            },
            (),
        );
    }

    let onsubmit = {
        let user = User {
            id: *id,
            username: username.clone(),
            email: to_option((*email).clone()),
            display_name: to_option((*display_name).clone()),
            can_login: *can_login,
            admin: *admin,
        };
        let history = use_history().unwrap();
        let path = format!("/api/users/{}", username);
        submit_form(path, user, history, Route::Users)
    };

    let delete_action = {
        let history = use_history().unwrap();
        let path = format!("/api/users/{}", username);
        ondelete(path, history, Route::Users)
    };

    html! {
        <CheckAuth admin=true>
            <div class="container my-5 mx-auto">
                <Form title="Edit User" {onsubmit} >
                    <h6 class={FORM_SUBTITLE}>
                        { format!("Username: {}", props.username.clone())}
                    </h6>
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
                    <DeleteButton
                        value="Delete User"
                        route={Route::Users}
                        show_modal={show_modal.clone()}
                    />
                    {" "}
                    <RouteButton value="Set Password" route={Route::SetPassword { username: props.username.clone() }} />
                    {" "}
                    <RouteButton value="Cancel" route={Route::Users} />
                <Modal
                    title="Delete User"
                    msg="Are you sure you want to delete this user? All assignments for with this user is assigned will also be deleted."
                    confirm_action={delete_action}
                    {show_modal}
                />
                </Form>
            </div>
        </CheckAuth>
    }
}

#[function_component(UserTable)]
pub fn user_table() -> Html {
    let users = use_state(Vec::<User>::new);

    // Get users on load
    {
        let users = users.clone();
        use_effect_with_deps(
            move |_| {
                onload("/api/users".into(), users);
                || ()
            },
            (),
        );
    }

    // Create table rows
    let rows = users.iter().map(|user| {
        let display_name = user
            .display_name
            .clone()
            .unwrap_or_else(|| user.username.clone());
        let email = user.email.clone();

        html_nested! {
            <Row>
                <CellLink value={display_name} route={Route::UserDetails { username: user.username.clone() }}/>
                <Cell value={email.unwrap_or_else(||"-".into())} />
            </Row>
        }
    });

    html! {
        <CheckAuth>
            <div class="container my-5 mx-auto max-w-3xl">
                <TableCard
                    title="Users"
                    headings={vec!["User", "Email"]}
                    button_label="Add User"
                    button_route={Route::AddUser}
                >
                    { for rows }
                </TableCard>
            </div>
        </CheckAuth>
    }
}

#[function_component(SetPassword)]
pub fn set_password(props: &UserProps) -> Html {
    let password = use_state(String::new);
    let password2 = use_state(String::new);

    let onsubmit = {
        if (*password).clone() == (*password2).clone() {
            let new_password = SetPasswdPayload {
                new_password: (*password).clone(),
            };
            let history = use_history().unwrap();
            let path = format!("/api/users/{}/set-password", props.username);
            submit_form(path, new_password, history, Route::Users)
        } else {
            Callback::from(move |e: FocusEvent| {
                e.prevent_default();
                notify_error("Passwords do not match");
            })
        }
    };

    html! {
        <CheckAuth admin=true>
            <div class="container my-5 mx-auto">
                <Form title="Set Password" subtitle={props.username.clone()} {onsubmit} >
                    <PasswordField
                        label="Password"
                        state={password}
                        required=true
                    />
                    <PasswordField
                        label="Re-Enter Password"
                        state={password2}
                        required=true
                    />
                    <Button
                        value="Set Password"
                        button_type={ButtonType::Primary}
                    />
                    {" "}
                    <RouteButton value="Cancel" route={Route::Users} />
                </Form>
            </div>
        </CheckAuth>
    }
}

#[function_component(UserDetails)]
pub fn user_details(props: &UserProps) -> Html {
    let user = use_state(User::default);
    let assignments = use_state(Vec::new);

    {
        let user = user.clone();
        let assignments = assignments.clone();
        let user_url = format!("/api/users/{}", &props.username);
        let user_keys_url = format!("/api/assignments?user={}", &props.username);
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match get::<User>(user_url).await {
                        Ok(u) => {
                            user.set(u);
                        }
                        Err(e) => notify_error(&e.to_string()),
                    };
                    match get::<Vec<Assignment>>(user_keys_url).await {
                        Ok(k) => {
                            assignments.set(k);
                        }
                        Err(e) => notify_error(&e.to_string()),
                    };
                });
                || ()
            },
            (),
        );
    }

    let user = (*user).clone();
    html! {
        <CheckAuth>
            <DetailsCard
                title={user.display_name.unwrap_or_else(|| user.username.clone())}
                edit_route={Route::EditUser { username: user.username.clone() }}
            >
                <DetailsHeader>
                    <DetailsHeaderItem content={format!("Email: {}", user.email.unwrap_or_else(|| "-".into()))} />
                    <DetailsHeaderItem content={format!("Can login: {}", user.can_login)} />
                    <DetailsHeaderItem content={format!("Admin: {}", user.admin)} />
                </DetailsHeader>
                <DetailsList label="Keys Assigned">
                    {
                        for (*assignments).iter().map(|a|
                            html_nested!{
                                <DetailsListItem
                                    label={
                                        if a.date_in.is_some() {
                                            format!("{} (Returned)", a.clone().key)
                                        } else {
                                            a.clone().key
                                        }
                                    }
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
