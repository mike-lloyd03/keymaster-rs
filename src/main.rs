use yew::prelude::*;
use yew_router::prelude::*;

mod nav;
mod routes;

use nav::Nav;
use routes::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Nav />
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
