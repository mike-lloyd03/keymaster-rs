use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod error;
mod nav;
mod routes;
mod services;
mod types;

use components::notifier::Notifier;
use components::session_provider::SessionProvider;
use nav::Nav;
use routes::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <SessionProvider>
            // <SessionMonitor />
            <BrowserRouter>
                <Nav />
                <Switch<Route> render={Switch::render(switch)} />
                <Notifier />
            </BrowserRouter>
        </SessionProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
