use yew::prelude::*;
use yew_router::prelude::*;
mod assignments;
mod keys;
mod login;
mod users;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/logout")]
    Logout,
    #[at("/keys")]
    GetAllKeys,
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
    #[at("/edit-user")]
    EditUser,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Home)]
fn home() -> Html {
    html! {
        <div>
            <h1>{ "This is home" }</h1>
        </div>
    }
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <login::Login /> },
        Route::Logout => todo!(),
        Route::GetAllKeys => html! { <keys::KeyTable /> },
        Route::AddKey => html! { <keys::NewKey />},
        Route::EditKey { key_name } => html! { <keys::EditKey key_name={ key_name.clone() }/>},
        Route::Assignments => html! { <assignments::Assignments />},
        Route::AssignKey => html! { <assignments::NewAssignment />},
        Route::EditAssignment => html! { <assignments::EditAssignment />},
        Route::Users => html! { <users::UserTable /> },
        Route::AddUser => html! { <users::NewUser />},
        Route::EditUser => html! { <users::EditUser />},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
