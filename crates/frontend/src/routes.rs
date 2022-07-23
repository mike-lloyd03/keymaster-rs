use yew::prelude::*;
use yew_router::prelude::*;
mod login;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/logout")]
    Logout,
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
        Route::AddKey => todo!(),
        Route::EditKey => todo!(),
        Route::Assignments => todo!(),
        Route::AssignKey => todo!(),
        Route::EditAssignment => todo!(),
        Route::Users => todo!(),
        Route::AddUser => todo!(),
        Route::EditUser => todo!(),
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
