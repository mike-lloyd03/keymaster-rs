use yew::prelude::*;
use yew_router::prelude::*;

mod assignments;
mod auth;
mod home;
mod keys;
mod users;

use assignments::*;
use keys::*;
use users::*;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Base,
    #[at("/home")]
    Home,
    #[at("/login")]
    Login,
    #[at("/logout")]
    Logout,
    #[at("/keys")]
    Keys,
    #[at("/add-key")]
    AddKey,
    #[at("/edit-key/:key_name")]
    EditKey { key_name: String },
    #[at("/keys/:key_name")]
    KeyDetails { key_name: String },
    #[at("/assignments")]
    Assignments,
    #[at("/assign-key")]
    AssignKey,
    #[at("/edit-assignment/:id")]
    EditAssignment { id: i64 },
    #[at("/assignments/:id")]
    AssignmentDetails { id: i64 },
    #[at("/users")]
    Users,
    #[at("/add-user")]
    AddUser,
    #[at("/edit-user/:username")]
    EditUser { username: String },
    #[at("/users/:username")]
    UserDetails { username: String },
    #[at("/edit-user/:username/set-password")]
    SetPassword { username: String },
    #[not_found]
    #[at("/not-found")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Base => html! { <Redirect<Route> to={Route::Home}/> },
        Route::Home => html! { <home::Home />},
        Route::Login => html! { <auth::Login /> },
        Route::Logout => html! { <auth::Logout /> },

        Route::Keys => html! { <KeyTable /> },
        Route::AddKey => html! { <NewKey />},
        Route::EditKey { key_name } => html! { <EditKey key_name={ key_name.clone() }/>},
        Route::KeyDetails { key_name } => html! { <KeyDetails key_name={ key_name.clone() }/>},

        Route::Assignments => html! { <Assignments />},
        Route::AssignKey => html! { <NewAssignment />},
        Route::EditAssignment { id } => html! { <EditAssignment id={ id.clone() }/>},
        Route::AssignmentDetails { id } => {
            html! {<AssignmentDetails id={ id.clone() }/>}
        }

        Route::Users => html! { <UserTable /> },
        Route::AddUser => html! { <NewUser />},
        Route::EditUser { username } => html! { <EditUser username={username.clone()}/>},
        Route::UserDetails { username } => {
            html! { <UserDetails username={username.clone()}/>}
        }
        Route::SetPassword { username } => {
            html! { <SetPassword username={username.clone()}/>}
        }

        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
