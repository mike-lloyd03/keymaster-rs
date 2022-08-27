use crate::components::table::{Cell, Row, Table};
use crate::routes::Route;
use crate::types::Assignment;
use crate::{routes::auth::CheckAuth, services::form_actions::onload_all};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct HomeQuery {
    sort: String,
}

#[function_component(Home)]
pub fn home() -> Html {
    let assignments = use_state(Vec::<Assignment>::new);
    let sorted_assignments = use_state(Vec::<SortedAssignment>::new);
    let headers = use_state(|| ("User", "Keys Assigned"));

    // Get assignments on load
    {
        let assignments = assignments.clone();
        use_effect_with_deps(
            move |_| {
                onload_all("/api/assignments".into(), assignments.clone());
                || ()
            },
            (),
        );
    }

    {
        let assignments_clone = assignments.clone();
        let assignments = assignments.clone();
        let sorted_assignments = sorted_assignments.clone();
        use_effect_with_deps(
            move |_| {
                sorted_assignments.set(sort_by_user((*assignments).clone()));
                || ()
            },
            assignments_clone,
        );
    }

    let on_sort_by_user = {
        let headers = headers.clone();
        let assignments = assignments.clone();
        let sorted_assignments = sorted_assignments.clone();
        Callback::from(move |_: MouseEvent| {
            sorted_assignments.set(sort_by_user((*assignments).clone()));
            headers.set(("User", "Keys Assigned"))
        })
    };

    let on_sort_by_key = {
        let headers = headers.clone();
        let assignments = assignments.clone();
        let sorted_assignments = sorted_assignments.clone();
        Callback::from(move |_: MouseEvent| {
            sorted_assignments.set(sort_by_key((*assignments).clone()));
            headers.set(("Key", "Users Assigned"))
        })
    };

    let rows = {
        let headers = headers.clone();
        let index_header = (*headers).clone().0;
        let values_header = (*headers).clone().1;
        sorted_assignments.iter().map(|a| {
            html_nested! {
                <Row>
                    <Cell heading={index_header.to_string()} value={a.index.clone()} />
                    <Cell heading={values_header.to_string()} value={a.values.clone()} />
                    </Row>
            }
        })
    };

    html! {
        <CheckAuth>
            <div class="container text-light my-3">
                <div class="row justify-content-center">
                    <div class="container py-2">
                        {"Sort:"}
                        <button
                            onclick={on_sort_by_key}
                            class={classes!("btn", "btn-primary")}
                        >
                            {"By Key"}
                        </button>
                        <button
                            onclick={on_sort_by_user}
                            class={classes!("btn", "btn-primary")}
                        >
                            {"By User"}
                        </button>
                    </div>
                    <Table title="Key Inventory Tracker">
                        {for rows}
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

#[derive(Ord, PartialEq, PartialOrd, Eq)]
struct SortedAssignment {
    index: String,
    values: String,
}

// impl Ord for SortedAssignment {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         todo!()
//     }
// }

fn sort_by_user(assignments: Vec<Assignment>) -> Vec<SortedAssignment> {
    let mut map = HashMap::new();
    for a in assignments {
        map.entry(a.user)
            .and_modify(|v| *v = format!("{}, {}", v, a.key))
            .or_insert(a.key);
    }
    map_to_sort(map)
}

fn sort_by_key(assignments: Vec<Assignment>) -> Vec<SortedAssignment> {
    let mut map = HashMap::new();
    for a in assignments {
        map.entry(a.key)
            .and_modify(|v| *v = format!("{}, {}", v, a.user))
            .or_insert(a.user);
    }
    map_to_sort(map)
}

fn map_to_sort(map: HashMap<String, String>) -> Vec<SortedAssignment> {
    let mut s: Vec<SortedAssignment> = map
        .iter()
        .map(|(k, v)| SortedAssignment {
            index: k.to_string(),
            values: v.to_string(),
        })
        .collect();
    s.sort();
    s
}
