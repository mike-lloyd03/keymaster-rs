use yew::prelude::*;
use yew_router::prelude::*;
mod login;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
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

// #[function_component(Login)]
// fn login() -> Html {
//     html! {
//         <div>
//             <h1>{ "Login" }</h1>
//         </div>
//     }
// }

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <login::Login /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
