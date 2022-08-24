use crate::components::form::{DateField, Form, MultiSelectField, MultiSelectOption, TextField};
use crate::components::table::{Cell, Row, Table};
use crate::services::form_actions::oninput_string;
use crate::types::Assignment;
use chrono::NaiveDate;
use yew::prelude::*;
use yew_router::prelude::*;

use super::Route;

#[function_component(NewAssignment)]
pub fn new_assignment() -> Html {
    let assignments = use_state(std::vec::Vec::new);
    let users = use_state(String::new);
    let keys = use_state(String::new);
    let date_out = use_state(String::new);
    let oninput_users = oninput_string(users.clone());
    let oninput_keys = oninput_string(keys.clone());
    let oninput_date_out = oninput_string(date_out.clone());

    let oncancel = {
        let history = use_history().unwrap();
        Callback::once(move |_: MouseEvent| history.push(Route::Assignments))
    };

    let onsubmit = {
        let assgn = Assignment {
            name: (*name).clone(),
            description: Some((*description).clone()),
            active: true,
        };
        let (_, dispatch) = use_store::<Notification>();
        let history = use_history().unwrap();
        onsubmit("/api/keys".to_string(), key, dispatch, history, Route::Keys)
    };

    let user_options = users.iter().map(|user| {
        html_nested! {
            <MultiSelectOption value={ user.0 } label={ user.1 } />
        }
    });

    let keys = vec!["key1", "keyToo", "keyTree"];
    let key_options = keys.iter().map(|key| {
        html_nested! {
            <MultiSelectOption value={ key.to_string() } />
        }
    });

    html! {
        <Form title="Assign Key">
            <MultiSelectField label="User">
                { for user_options }
            </MultiSelectField>
            <MultiSelectField label="Key">
                { for key_options }
            </MultiSelectField>
            <DateField label="Date Out" />
        </Form>
    }
}

#[function_component(EditAssignment)]
pub fn edit_assignment() -> Html {
    html! {
        <Form title="Edit Assignment">
            <TextField label="User" />
            <TextField label="Key" />
            <DateField label="Date Out" />
            <DateField label="Date In" />
        </Form>
    }
}
#[function_component(Assignments)]
pub fn assignments() -> Html {
    let assignments = vec![
        Assignment {
            user: "mike".to_string(),
            key: "key1".to_string(),
            date_out: NaiveDate::from_ymd(2013, 5, 3),
            ..Default::default()
        },
        Assignment {
            user: "aaron".to_string(),
            key: "key2".to_string(),
            date_out: NaiveDate::from_ymd(2016, 1, 23),
            ..Default::default()
        },
        Assignment {
            user: "johnny".to_string(),
            key: "key3".to_string(),
            date_out: NaiveDate::from_ymd(2045, 8, 12),
            date_in: Some(NaiveDate::from_ymd(2048, 12, 25)),
            ..Default::default()
        },
    ];

    let rows = assignments.iter().map(|a| {
        html_nested! {
            <Row>
                <Cell heading="User" value={
                    a.user.clone()
                } />
                <Cell heading="Key" value={
                    a.key.clone()
                } />
                <Cell heading="Date Out" value={
                    a.date_out.format("%Y-%m-%d").to_string()
                } />
                <Cell heading="Date In" value={
                    match a.date_in {
                        Some(d) => d.format("%Y-%m-%d").to_string(),
                        None => "".to_string(),
                    }
                } />
                <Cell heading="" value="Edit" />
            </Row>
        }
    });

    html! {
        <div class="container text-light my-3">
            <div class="row justify-content-center">
                <Table title="Assignments">
                { for rows }
                </Table>
            </div>
        </div>
    }
}
