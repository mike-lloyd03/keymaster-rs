use yew::prelude::*;
use yew_router::prelude::*;

// mod models;
mod components;
mod nav;
mod routes;

use nav::Nav;
use routes::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Nav />
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
