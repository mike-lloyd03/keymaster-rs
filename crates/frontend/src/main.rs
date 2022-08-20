use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod error;
mod hooks;
mod nav;
mod routes;
mod services;

use components::notifier::Notifier;
use components::user_context_provider::UserContextProvider;
use nav::Nav;
use routes::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <UserContextProvider>
                <Nav />
                <Switch<Route> render={Switch::render(switch)} />
                <Notifier />
            </UserContextProvider>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
