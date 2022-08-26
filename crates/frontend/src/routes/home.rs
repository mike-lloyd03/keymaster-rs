use crate::{
    components::notifier::notify_info, routes::auth::CheckAuth, services::form_actions::onload_all,
    types::Assignment,
};
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Deserialize, Debug)]
struct HomeQuery {
    sort: Option<String>,
}

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();
    let query = history.location().query::<HomeQuery>().unwrap();
    let assignments = use_state(Vec::<Assignment>::new);

    // notify_info(&format!("{:?}", query));

    // Get assignments on load
    {
        let assignments = assignments.clone();
        use_effect_with_deps(
            move |_| {
                onload_all("/api/assignments".into(), assignments);
                || ()
            },
            (),
        );
    }

    html! {
        // <CheckAuth>
            <div class="container text-light my-3">
                <div class="" style="text-align: center">
                    <h4>{"Key Inventory Tracker"}</h4>
                <div class="container py-2">
                {"Sort:"}
                <a class="btn btn-primary" href="/index?sort=by_user" role="button">{"By User"}</a>
                <a class="btn btn-primary" href="/index?sort=by_key" role="button">{"By Key"}</a>
                </div>
                    <div class="">
                        <table class="table table-striped table-hover table-bordered table-dark">
                            <thead class="table-dark">
                                <tr>
                                    <th>{"User"}</th>
                                    <th>{"Assigned Keys"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <td>{"Aaron Sum"}</td>
                                    <td>{"Finance Key, KeyM4"}</td>
                                </tr>
                                <tr>
                                    <td>{"Alec Parker"}</td>
                                    <td>{"Gate Key, Key0, Key02, KeyM4"}</td>
                                </tr>
                                <tr>
                                    <td>{"Bob Degraff"}</td>
                                    <td>{"KeyM4"}</td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        // </CheckAuth>
    }
}
