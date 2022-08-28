use crate::components::table::{Cell, Row, Table};
use crate::services::get_display_name;
use crate::types::{Assignment, User};
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
    let all_users = use_state(Vec::<User>::new);

    // Get assignments on load
    {
        let assignments = assignments.clone();
        let all_users = all_users.clone();
        use_effect_with_deps(
            move |_| {
                onload_all("/api/assignments".into(), assignments.clone());
                onload_all("/api/users".into(), all_users);
                || ()
            },
            (),
        );
    }

    {
        let assignments_clone = assignments.clone();
        let assignments = assignments.clone();
        let sorted_assignments = sorted_assignments.clone();
        let all_users = all_users.clone();
        use_effect_with_deps(
            move |_| {
                sorted_assignments.set(agg_by_user(&*assignments, &*all_users));
                || ()
            },
            assignments_clone,
        );
    }

    let on_sort_by_user = {
        let headers = headers.clone();
        let assignments = assignments.clone();
        let sorted_assignments = sorted_assignments.clone();
        let all_users = all_users.clone();
        Callback::from(move |_: MouseEvent| {
            sorted_assignments.set(agg_by_user(&*assignments, &*all_users));
            headers.set(("User", "Keys Assigned"))
        })
    };

    let on_sort_by_key = {
        let headers = headers.clone();
        let assignments = assignments.clone();
        let sorted_assignments = sorted_assignments.clone();
        let all_users = all_users.clone();
        Callback::from(move |_: MouseEvent| {
            sorted_assignments.set(agg_by_key(&*assignments, &*all_users));
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
    }
}

#[derive(Ord, PartialEq, PartialOrd, Eq)]
struct SortedAssignment {
    index: String,
    values: String,
}

/// Aggregates the assignment list by user
fn agg_by_user(assignments: &Vec<Assignment>, users: &Vec<User>) -> Vec<SortedAssignment> {
    let mut map = HashMap::new();
    let assignments = assignments.clone();
    for a in assignments {
        map.entry(get_display_name(&users, a.user))
            .and_modify(|v| *v = format!("{}, {}", v, a.key))
            .or_insert(a.key);
    }
    map_to_sort(map)
}

/// Aggregates the assignment list by key
fn agg_by_key(assignments: &Vec<Assignment>, users: &Vec<User>) -> Vec<SortedAssignment> {
    let mut map = HashMap::new();
    let assignments = assignments.clone();
    for a in assignments {
        map.entry(a.key)
            .and_modify(|v| *v = format!("{}, {}", v, get_display_name(users, a.user.clone())))
            .or_insert(get_display_name(users, a.user));
    }
    map_to_sort(map)
}

/// Converts a hashmap into a vector of SortedAssignment
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
