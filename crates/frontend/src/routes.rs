use yew::prelude::*;
use yew_router::prelude::*;
mod forms;
mod keys;
mod login;
mod new_item;
mod table;
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
    #[at("/edit-key")]
    EditKey,
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
        Route::GetAllKeys => html! { <keys::Keys /> },
        Route::AddKey => html! { <new_item::NewKey />},
        Route::EditKey => html! { <new_item::EditKey />},
        Route::Assignments => todo!(),
        Route::AssignKey => html! { <new_item::NewAssignment />},
        Route::EditAssignment => html! { <new_item::EditAssignment />},
        Route::Users => html! { <users::Users /> },
        Route::AddUser => html! { <new_item::NewUser />},
        Route::EditUser => html! { <new_item::EditUser />},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
