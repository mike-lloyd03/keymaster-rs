use crate::components::form::{CheckboxField, Form, TextField};
use crate::components::table::{Cell, Row, Table};
use yew::prelude::*;

#[derive(PartialEq, Default, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub can_login: bool,
    pub admin: bool,
}

#[function_component(NewUser)]
pub fn new_user() -> Html {
    html! {
        <Form title="New User" submit_label="User">
            <TextField label="Username" />
            <TextField label="Email" />
            <TextField label="Display Name" />
            <CheckboxField label="Can Login?" />
        </Form>
    }
}

#[function_component(EditUser)]
pub fn edit_user() -> Html {
    html! {
        <Form title="Edit User" submit_label="Save Changes">
            <TextField label="Email" />
            <TextField label="Display Name" />
            <CheckboxField label="Can Login?" />
        </Form>
    }
}

#[function_component(UserTable)]
pub fn user_table() -> Html {
    let users = vec![
        User {
            username: "mike".to_string(),
            display_name: Some("Mike Morc".to_string()),
            email: Some("Nope@email.com".to_string()),
            ..Default::default()
        },
        User {
            username: "aaron".to_string(),
            display_name: Some("Aaron Plus".to_string()),
            email: Some("cali_sucks@leaving.com".to_string()),
            ..Default::default()
        },
        User {
            username: "johnny".to_string(),
            ..Default::default()
        },
    ];

    let rows = users.iter().map(|user| {
        html_nested! {
            <Row>
                <Cell heading="User" value={
                    user.display_name.clone().unwrap_or_else(|| user.username.clone())
                } />
                <Cell heading="Email" value={
                    user.email.clone().unwrap_or_else(|| "".to_string())
                } />
                <Cell heading="" value="Edit" />
            </Row>
        }
    });

    html! {
        <div class="container text-light my-3">
            <div class="row justify-content-center">
                <Table title="Users">
                { for rows }
                </Table>
            </div>
        </div>
    }
}
