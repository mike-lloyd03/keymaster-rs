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

#[derive(Properties, PartialEq)]
pub struct UserListProps {
    users: Vec<User>,
}

#[function_component(UserList)]
pub fn user_list(props: &UserListProps) -> Html {
    props
        .users
        .iter()
        .map(|user| {
            let user = user.clone();

            html! {
                <tr>
                    <td>{ user.display_name.unwrap_or_else(|| user.username.to_string()) }</td>
                    <td>{ user.email.unwrap_or_else(|| "".to_string()) }</td>
                    <td>
                        <a class="btn btn-outline-primary" href="#" role="button">{ "Edit" }</a>
                    </td>
                </tr>
            }
        })
        .collect()
}

#[function_component(Users)]
pub fn users() -> Html {
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
    html! {
        <div class="container text-light my-3">
            <div class="row justify-content-center">
                <div style="text-align: center">
                    <h2>{"Users"}</h2>
                    <div class="container py-2">
                        <a class="btn btn-primary" href="/add-user" role="button">{ "Add User" }</a>
                    </div>
                    <table class="table table-striped table-hover table-bordered table-dark">
                        <thead class="table-dark">
                            <tr>
                                <th>{ "User" }</th>
                                <th>{ "Email" }</th>
                                <th></th>
                            </tr>
                        </thead>
                        <tbody>
                            <UserList users={ users } />
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}
