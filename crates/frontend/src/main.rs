use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod error;
mod nav;
mod routes;
mod services;
mod types;

use components::notifier::Notifier;
use nav::Nav;
use routes::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <BrowserRouter>
            <Nav />
            <Switch<Route> render={Switch::render(switch)} />
            <Notifier />
        </BrowserRouter>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    let document = web_sys::window().unwrap().document().unwrap();
    let yew = document.query_selector("#yew").unwrap().unwrap();
    yew::start_app_in_element::<App>(yew);
}
