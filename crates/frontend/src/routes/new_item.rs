use yew::prelude::*;

use crate::routes::forms::{
    CheckboxField, DateField, Form, MultiSelectField, MultiSelectOption, TextField,
};

#[function_component(NewKey)]
pub fn new_key() -> Html {
    html! {
        <Form title="New Key" action_label="Key">
            <TextField label="Key Name" />
            <TextField label="Description" />
        </Form>
    }
}

#[function_component(EditKey)]
pub fn edit_key() -> Html {
    html! {
        <Form title="Edit Key" action_label="Save Changes">
            <TextField label="Description" />
            <CheckboxField label="Active" />
        </Form>
    }
}

#[function_component(NewUser)]
pub fn new_user() -> Html {
    html! {
        <Form title="New User" action_label="User">
            <TextField label="Username" />
            <TextField label="Email" />
            <TextField label="Display Name" />
            <CheckboxField label="Can Login?" />
        </Form>
    }
}

#[function_component(EditUser)]
pub fn edit_user() -> Html {
    html! {
        <Form title="Edit User" action_label="Save Changes">
            <TextField label="Email" />
            <TextField label="Display Name" />
            <CheckboxField label="Can Login?" />
        </Form>
    }
}

#[function_component(NewAssignment)]
pub fn new_assignment() -> Html {
    let users: Html = vec![("gunther", "Gunther Morrison"), ("matt", "Matt Chandler")]
        .iter()
        .map(|user| {
            html! {
                <MultiSelectOption value={ user.0 } label={ user.1 } />
            }
        })
        .collect();

    let keys: Html = vec!["key1", "keyToo", "keyTree"]
        .iter()
        .map(|key| {
            html! {
                <MultiSelectOption value={ key.to_string() } />
            }
        })
        .collect();

    html! {
        <Form title="Assign Key" action_label="Assign Key">
            <MultiSelectField label="User">
                { users }
            </MultiSelectField>
            <MultiSelectField label="Key">
                { keys }
            </MultiSelectField>
            <DateField label="Date Out" />
        </Form>
    }
}

#[function_component(EditAssignment)]
pub fn edit_assignment() -> Html {
    html! {
        <Form title="Edit Assignment" action_label="Save Changes">
            <TextField label="User" />
            <TextField label="Key" />
            <DateField label="Date Out" />
            <DateField label="Date In" />
        </Form>
    }
}
