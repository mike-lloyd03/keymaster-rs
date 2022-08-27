use crate::components::notifier::notify_info;
use crate::components::table::{Cell, Row, Table};
use crate::routes::Route;
use crate::{routes::auth::CheckAuth, services::form_actions::onload_all};

use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct HomeQuery {
    sort: String,
}

#[derive(Serialize, Deserialize)]
pub struct SortedAssignments {
    index: String,
    values: String,
}

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();
    let assignments = use_state(Vec::<SortedAssignments>::new);
    let query = history.location().query::<HomeQuery>();

    let sort_by = match query {
        Ok(q) if q.sort == "by_key".to_string() => "by_key",
        _ => "by_user",
    };

    notify_info(sort_by);

    // Get assignments on load
    {
        let assignments = assignments.clone();
        let url = format!("/api/assignments?sort={}", sort_by);
        use_effect_with_deps(
            move |_| {
                onload_all(url, assignments);
                || ()
            },
            (),
        );
    }

    let rows = assignments.iter().map(|a| {
        html_nested! {
            <Row>
                <Cell heading="Index" value={a.index.clone()} />
                <Cell heading="Values" value={a.values.clone()} />
            </Row>
        }
    });

    log::info!("Mounting Home");
    html! {
        <CheckAuth>
            <div class="container text-light my-3">
                <div class="row justify-content-center">
                    <div class="container py-2">
                        {"Sort:"}
                        <Link<Route, HomeQuery>
                            to={Route::Home}
                            query={Some(HomeQuery { sort: "by_key".into() })}
                            classes={classes!("btn", "btn-primary")}
                        >
                            {"By Key"}
                        </Link<Route, HomeQuery>>
                        <Link<Route, HomeQuery>
                            to={Route::Home}
                            query={Some(HomeQuery { sort: "by_user".into() })}
                            classes={classes!("btn", "btn-primary")}
                        >
                            {"By User"}
                        </Link<Route, HomeQuery>>
                    </div>
                    <Table title="Key Inventory Tracker">
                    { for rows }
                    </Table>
                </div>
            </div>
        </CheckAuth>
        // <CheckAuth>
        //     <div class="container text-light my-3">
        //         <div class="" style="text-align: center">
        //             <h4>{"Key Inventory Tracker"}</h4>
        //         <div class="container py-2">
        //         {"Sort:"}
        //         <a class="btn btn-primary" href="/index?sort=by_user" role="button">{"By User"}</a>
        //         <a class="btn btn-primary" href="/index?sort=by_key" role="button">{"By Key"}</a>
        //         </div>
        //             <div class="">
        //                 <table class="table table-striped table-hover table-bordered table-dark">
        //                     <thead class="table-dark">
        //                         <tr>
        //                             <th>{"User"}</th>
        //                             <th>{"Assigned Keys"}</th>
        //                         </tr>
        //                     </thead>
        //                     <tbody>
        //                         <tr>
        //                             <td>{"Aaron Sum"}</td>
        //                             <td>{"Finance Key, KeyM4"}</td>
        //                         </tr>
        //                         <tr>
        //                             <td>{"Alec Parker"}</td>
        //                             <td>{"Gate Key, Key0, Key02, KeyM4"}</td>
        //                         </tr>
        //                         <tr>
        //                             <td>{"Bob Degraff"}</td>
        //                             <td>{"KeyM4"}</td>
        //                         </tr>
        //                     </tbody>
        //                 </table>
        //             </div>
        //         </div>
        //     </div>
        // </CheckAuth>
    }
}
