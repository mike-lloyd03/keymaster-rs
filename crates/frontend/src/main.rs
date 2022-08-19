use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod nav;
mod routes;

use components::notifier::Notifier;
use nav::Nav;
use routes::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Nav />
            <Switch<Route> render={Switch::render(switch)} />
            <Notifier />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
