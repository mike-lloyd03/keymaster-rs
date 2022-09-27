use std::vec::Vec;

use crate::components::details_card::*;
use crate::components::form::*;
use crate::components::modal::Modal;
use crate::components::notifier::notify_error;
use crate::components::table::*;
use crate::services::form_actions::{get_options, ondelete, onload, submit_form};
use crate::services::requests::get;
use crate::services::{format_date, get_display_name, parse_date};
use crate::theme::FORM_SUBTITLE;
use crate::types::{Assignment, User};

use yew::prelude::*;
use yew_router::prelude::*;

use super::auth::CheckAuth;
use super::Route;

#[function_component(NewAssignment)]
pub fn new_assignment() -> Html {
    let available_users = use_state(Vec::new);
    let available_keys = use_state(Vec::new);
    let date_out = use_state(String::new);
    let selected_users = use_state(Vec::<String>::new);
    let selected_keys = use_state(Vec::<String>::new);
    let all_users = use_state(Vec::<User>::new);

    {
        let users = available_users.clone();
        let keys = available_keys.clone();
        let all_users = all_users.clone();
        use_effect_with_deps(
            move |_| {
                get_options(users, keys);
                onload("/api/users".into(), all_users);
                || ()
            },
            (),
        );
    }

    let onsubmit = {
        let mut assignments: Vec<Assignment> = Vec::new();
        let date_out = parse_date((*date_out).clone());

        for user in &*selected_users {
            for key in &*selected_keys {
                let a = Assignment {
                    user: user.into(),
                    key: key.into(),
                    date_out,
                    ..Default::default()
                };
                assignments.push(a);
            }
        }
        let history = use_history().unwrap();
        submit_form(
            "/api/assignments".to_string(),
            assignments,
            history,
            Route::Assignments,
        )
    };

    let user_options = available_users.iter().map(|user| {
        html_nested! {
            <MultiSelectOption label={ get_display_name(&(*all_users), user.clone()) } value={ user.clone() } />
        }
    });

    let key_options = available_keys.iter().map(|key| {
        html_nested! {
            <MultiSelectOption value={key.to_string()} />
        }
    });

    html! {
        <CheckAuth admin=true>
            <div class="container my-5 mx-auto">
                <Form title="Assign Key" {onsubmit}>
                    <MultiSelectField label="User" state={selected_users}>
                        { for user_options.clone() }
                    </MultiSelectField>
                    <MultiSelectField label="Key" state={selected_keys}>
                        { for key_options }
                    </MultiSelectField>
                    <DateField label="Date Out" required=true state={date_out} />
                    <Button value="Assign Key" button_type={ButtonType::Primary} />
                    {" "}
                    <RouteButton value="Cancel" route={Route::Assignments} />
                </Form>
            </div>
        </CheckAuth>
    }
}

#[derive(PartialEq, Eq, Properties)]
pub struct AssignmentProps {
    pub id: i64,
}

#[function_component(EditAssignment)]
pub fn edit_assignment(props: &AssignmentProps) -> Html {
    let user = use_state(String::new);
    let key = use_state(String::new);
    let date_out = use_state(String::new);
    let date_in = use_state(String::new);

    let show_modal = use_state(|| false);
    let users = use_state(Vec::<User>::new);

    {
        let user = user.clone();
        let key = key.clone();
        let date_out = date_out.clone();
        let date_in = date_in.clone();
        let users = users.clone();
        let url = format!("/api/assignments/{}", &props.id.clone());
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match get::<Assignment>(url).await {
                        Ok(a) => {
                            user.set(a.user);
                            key.set(a.key);
                            date_out.set(format_date(a.date_out));
                            date_in.set(match a.date_in {
                                Some(d) => format_date(d),
                                None => "".into(),
                            });
                        }
                        Err(e) => notify_error(&e.to_string()),
                    }
                });
                onload("/api/users".into(), users);
                || ()
            },
            (),
        );
    }

    let onsubmit = {
        let assignment = Assignment {
            user: (*user).clone(),
            key: (*key).clone(),
            date_out: parse_date((*date_out).clone()),
            date_in: if (*date_in).clone().is_empty() {
                None
            } else {
                Some(parse_date((*date_in).clone()))
            },
            ..Default::default()
        };
        let history = use_history().unwrap();
        let path = format!("/api/assignments/{}", props.id);
        submit_form(path, assignment, history, Route::Assignments)
    };

    let delete_action = {
        let history = use_history().unwrap();
        let path = format!("/api/assignments/{}", props.id);
        ondelete(path, history, Route::Assignments)
    };

    html! {
        <CheckAuth admin=true>
            <div class="container my-5 mx-auto">
                <Form title="Edit Assignment" {onsubmit}>
                    <h6 class={FORM_SUBTITLE}>
                        { format!("User: {}", get_display_name(&(*users), (*user).clone())) }
                    </h6>
                    <h6 class={FORM_SUBTITLE}>
                        { format!("Key: {}", (*key).clone()) }
                    </h6>
                    <DateField label="Date Out" state={date_out}/>
                    <DateField label="Date In" state={date_in}/>
                    <Button
                        value="Update Assignment"
                        button_type={ButtonType::Primary}
                    />
                    {" "}
                    <DeleteButton
                        value="Delete Assignment"
                        route={Route::Assignments}
                        show_modal={show_modal.clone()}
                    />
                    {" "}
                    <RouteButton value="Cancel" route={Route::Assignments}/>
                </Form>
                <Modal
                    title="Delete Assignment"
                    msg="Are you sure you want to delete this assignment?"
                    confirm_action={delete_action}
                    {show_modal}
                />
            </div>
        </CheckAuth>
    }
}
#[function_component(Assignments)]
pub fn assignments() -> Html {
    let assignments = use_state(Vec::<Assignment>::new);
    let all_users = use_state(Vec::<User>::new);

    // Get assignments on load
    {
        let assignments = assignments.clone();
        let all_users = all_users.clone();
        use_effect_with_deps(
            move |_| {
                onload("/api/assignments".into(), assignments);
                onload("/api/users".into(), all_users);
                || ()
            },
            (),
        );
    }

    let rows = assignments.iter().map(|a| {
        html_nested! {
            <Row>
                <CellLink
                    value={get_display_name(&all_users, a.user.clone())}
                    route={Route::AssignmentDetails { id: a.id }}
                />
                <Cell value={a.key.clone()} />
                <Cell value={format_date(a.date_out)} />
                <Cell value={
                    match a.date_in {
                        Some(d) => format_date(d),
                        None => "-".to_string(),
                    }
                } />
            </Row>
        }
    });

    html! {
        <CheckAuth>
            <div class="container my-5 mx-auto max-w-4xl">
                <TableCard
                    title="Assignments"
                    headings={vec!["User", "Key", "Date Out", "Date In"]}
                    button_label="Assign Key"
                    button_route={Route::AssignKey}
                >
                    { for rows }
                </TableCard>
            </div>
        </CheckAuth>
    }
}

#[function_component(AssignmentDetails)]
pub fn assignment_details(props: &AssignmentProps) -> Html {
    let assignment = use_state(|| Assignment::default());
    let users = use_state(Vec::<User>::new);

    {
        let assignment = assignment.clone();
        let users = users.clone();
        let assignment_url = format!("/api/assignments/{}", &props.id);
        use_effect_with_deps(
            move |_| {
                onload(assignment_url, assignment);
                onload("/api/users".into(), users);
                || ()
            },
            (),
        );
    }

    let assignment = (*assignment).clone();
    html! {
        <CheckAuth>
            <DetailsCard
                title={format!("Assignment {}", assignment.id)}
                edit_route={Route::EditAssignment { id: assignment.id }}
            >
                <DetailsHeader>
                    <DetailsHeaderItem content={format!("Assigned to: {}", get_display_name(&(*users), assignment.user))} />
                    <DetailsHeaderItem content={format!("Key: {}", assignment.key)} />
                    <DetailsHeaderItem content={format!("Date assigned: {}", assignment.date_out)} />
                    <DetailsHeaderItem
                        content={
                            match assignment.date_in {
                                Some(d) => format!("Date returned: {}", d),
                                None => "Date returned: -".into(),
                            }
                        }
                    />
                </DetailsHeader>
                <DetailsFooter/>
            </DetailsCard>
        </CheckAuth>
    }
}
