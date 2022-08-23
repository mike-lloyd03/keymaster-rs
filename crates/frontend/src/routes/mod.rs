use yew::prelude::*;
use yew_router::prelude::*;

mod assignments;
mod auth;
mod keys;
mod users;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
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
    #[at("/assignments")]
    Assignments,
    #[at("/assign-key")]
    AssignKey,
    #[at("/edit-assignment")]
    EditAssignment,
    #[at("/users")]
    Users,
    #[at("/add-user")]
    AddUser,
    #[at("/edit-user/:username")]
    EditUser { username: String },
    #[not_found]
    #[at("/not-found")]
    NotFound,
}

#[function_component(Home)]
fn home() -> Html {
    html! {
            <div>
                <h1>{ "This is home" }</h1>
                <div class="dropdown">
      <button class="btn btn-secondary dropdown-toggle" type="button" data-toggle="dropdown" aria-expanded="false">
        {"Dropdown button"}
      </button>
      <div class="dropdown-menu">
        <a class="dropdown-item" href="#">{ "Action" }</a>
        <a class="dropdown-item" href="#">{ "Another action" }</a>
        <a class="dropdown-item" href="#">{ "Something else here" }</a>
      </div>
    </div>
            </div>
        }
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <auth::Login /> },
        Route::Logout => html! { <auth::Logout /> },
        Route::Keys => html! { <keys::KeyTable /> },
        Route::AddKey => html! { <keys::NewKey />},
        Route::EditKey { key_name } => html! { <keys::EditKey key_name={ key_name.clone() }/>},
        Route::Assignments => html! { <assignments::Assignments />},
        Route::AssignKey => html! { <assignments::NewAssignment />},
        Route::EditAssignment => html! { <assignments::EditAssignment />},
        Route::Users => html! { <users::UserTable /> },
        Route::AddUser => html! { <users::NewUser />},
        Route::EditUser { username } => html! { <users::EditUser username={username.clone()}/>},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
