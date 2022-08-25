use std::fmt::Display;

use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FormProps {
    pub title: String,
    pub subtitle: Option<String>,
    pub action: Option<String>,
    pub method: Option<String>,
    pub onsubmit: Option<Callback<FocusEvent>>,
    pub children: Children,
}

#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    html! {
        <div class="container text-light my-3" style="max-width: 600px;">
            <div class="row justify-content-center">
                <h1>{ props.title.clone() }</h1>
                {
                    match props.subtitle.clone() {
                        Some(s) => html!{<h4>{ s.to_string() }</h4>},
                        None => html!{},
                    }
                }
                <form action={
                        format!("http://localhost:8080/api/{}",
                        props.action.clone().unwrap_or_default()
                        )
                    }
                    id="form"
                    method="post"
                    class="form"
                    role="form"
                    onsubmit={ props.onsubmit.clone() }
                >
                    {
                        for props.children.iter()
                    }
                </form>
            </div>
        </div>
    }
}

#[derive(PartialEq, Clone)]
pub enum ButtonType {
    Primary,
    Secondary,
    Danger,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub value: String,
    pub button_type: Option<ButtonType>,
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <input
            class={
                match &props.button_type.clone().unwrap_or(ButtonType::Primary) {
                    ButtonType::Primary => "btn btn-primary",
                    ButtonType::Secondary => "btn btn-secondary",
                    ButtonType::Danger => "btn btn-danger",
                }
            }
            type="submit"
            value={ props.value.clone() }
            onclick={ props.onclick.clone() }
        />
    }
}

#[derive(PartialEq, Clone)]
pub enum InputType {
    Text,
    Date,
    Password,
}

impl Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::Text => write!(f, "text"),
            InputType::Date => write!(f, "date"),
            InputType::Password => write!(f, "password"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub input_type: Option<InputType>,
    pub label: String,
    pub state: UseStateHandle<String>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub oninput: Option<Callback<InputEvent>>,
    pub required: Option<bool>,
    pub pattern: Option<String>,
    pub checked: Option<bool>,
}

#[function_component(InputField)]
pub fn input_field(props: &InputProps) -> Html {
    let label_snake = snake_case(props.label.clone());

    let oninput = {
        let state = props.state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            state.set(input.value());
        })
    };

    html! {
        <div class="form-group ">
            <label class="control-label" for={ label_snake.clone() }>{ props.label.clone() }</label>
            <input
                class="form-control"
                name={ label_snake.clone() }
                type={ props.input_type.clone().unwrap().to_string() }
                value={(&*props.state).clone()}
                required={props.required.unwrap_or_default()}
                pattern={props.pattern.clone()}
                {oninput}
            />
        </div>

    }
}

#[function_component(TextField)]
pub fn text_field(props: &InputProps) -> Html {
    html! {
        <InputField
            input_type={InputType::Text}
            label={props.label.clone()}
            state={props.state.clone()}
            name={props.name.clone()}
            value={props.value.clone()}
            oninput={props.oninput.clone()}
            required={props.required}
            pattern={props.pattern.clone()}
        />
    }
}

#[function_component(DateField)]
pub fn date_field(props: &InputProps) -> Html {
    html! {
        <InputField
            input_type={InputType::Date}
            label={props.label.clone()}
            state={props.state.clone()}
            name={props.name.clone()}
            value={props.value.clone()}
            oninput={props.oninput.clone()}
            required={props.required}
        />
    }
}

#[function_component(PasswordField)]
pub fn password_field(props: &InputProps) -> Html {
    html! {
        <InputField
            input_type={InputType::Password}
            label={props.label.clone()}
            state={props.state.clone()}
            name={props.name.clone()}
            value={props.value.clone()}
            oninput={props.oninput.clone()}
            required={props.required}
        />
    }
}

#[derive(Properties, PartialEq)]
pub struct CheckboxProps {
    pub label: String,
    pub state: UseStateHandle<bool>,
}

#[function_component(CheckboxField)]
pub fn checkbox_field(props: &CheckboxProps) -> Html {
    let label = &props.label;

    let onchange = {
        let state = props.state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            state.set(input.checked());
        })
    };

    html! {
        <div class="checkbox">
            <label>
                <input
                    name={ snake_case(label.clone()) }
                    type="checkbox"
                    checked={*(props.state)}
                    {onchange}
                />
                { format!(" {}", label) }
            </label>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct MultiSelectFieldProps {
    pub label: String,
    pub state: UseStateHandle<Vec<String>>,
    pub children: ChildrenWithProps<MultiSelectOption>,
}

#[function_component(MultiSelectField)]
pub fn multi_select_field(props: &MultiSelectFieldProps) -> Html {
    let label_sn = snake_case(props.label.clone());

    let onchange = {
        let state = props.state.clone();
        Callback::from(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
                let collection = input.selected_options();
                let selected: Vec<String> = (0..input.selected_options().length())
                    .filter_map(|i| collection.item(i))
                    .filter_map(|e| e.text_content())
                    .collect();

                state.set(selected);
            }
        })
    };

    html! {
        <div class="form-group  required">
            <label class="control-label" for={ label_sn.clone() }>{ props.label.clone() }</label>
            <select
                class="form-control"
                id={ label_sn.clone() }
                multiple=true
                name={ label_sn.clone() }
                required=true
                {onchange}
            >
                {
                    for props.children.iter()
                }
            </select>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct MultiSelectOptionProps {
    pub value: String,
    pub label: Option<String>,
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(MultiSelectOption)]
pub fn multi_select_option(props: &MultiSelectOptionProps) -> Html {
    html! {
        <option value={ props.value.clone() } onclick={props.onclick.clone()}>{
            props.label.clone().unwrap_or_else(|| props.value.clone())
        }</option>
    }
}

/// Converts a normal case string to lower snake case
/// Example: snake_case("Date out".into()) -> "date_out"
fn snake_case(s: String) -> String {
    s.to_lowercase().replace(" ", "_")
}
