use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod error;
mod routes;
mod services;
mod theme;
mod types;

// use components::nav::Nav;
use components::nav2::Navbar;
use components::notifier::Notifier;
use components::session_provider::SessionProvider;
use routes::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <SessionProvider>
            <BrowserRouter>
                <Navbar />
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
