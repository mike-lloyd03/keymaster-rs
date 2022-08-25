use web_sys::HtmlInputElement;
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
    pub name: Option<String>,
    pub button_type: Option<ButtonType>,
    pub onclick: Option<Callback<MouseEvent>>,
    pub novalidate: Option<bool>,
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
            formnovalidate={ props.novalidate.unwrap_or_default() }
            id={ props.name.clone() }
            name={ props.name.clone() }
            type="submit"
            value={ props.value.clone() }
            onclick={ props.onclick.clone() }
        />
    }
}

#[derive(Properties, PartialEq)]
pub struct LabelProps {
    pub label: String,
    pub name: Option<String>,
    pub value: Option<String>,
    pub oninput: Option<Callback<InputEvent>>,
    pub required: Option<bool>,
    pub pattern: Option<String>,
    pub checked: Option<bool>,
    pub state: Option<UseStateHandle<String>>,
}

#[function_component(TextField)]
pub fn text_field(props: &LabelProps) -> Html {
    let label = &props.label;

    let oninput = {
        let state = props.state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(s) = state.clone() {
                s.set(input.value());
            }
        })
    };

    match &props.state {
        Some(s) => {
            html! {
                <div class="form-group ">
                    <label class="control-label" for={ snake_case(label.clone()) }>{ label }</label>
                    <input
                    class="form-control"
                    id={ snake_case(label.clone()) }
                    name={ props.name.clone().unwrap_or_else(|| snake_case(label.clone())) }
                    type="text"
                    value={(&**s).clone()}
                    required={props.required.unwrap_or_default()}
                    pattern={props.pattern.clone()}
                    {oninput}
                />
                    </div>

            }
        }
        None => html! {
        <div class="form-group ">
        <label class="control-label" for={ snake_case(label.clone()) }>{ label }</label>
            <input
                class="form-control"
                id={ snake_case(label.clone()) }
                name={ props.name.clone().unwrap_or_else(|| snake_case(label.clone())) }
                type="text"
                value={props.value.clone()}
                required={props.required.unwrap_or_default()}
                pattern={props.pattern.clone()}
                oninput={props.oninput.clone()}
            />
        </div>
        },
    }
}

#[function_component(DateField)]
pub fn date_field(props: &LabelProps) -> Html {
    let label = &props.label;

    let oninput = {
        let state = props.state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(s) = state.clone() {
                s.set(input.value());
            }
        })
    };

    // html! {
    //     <div class="form-group required">
    //         <label class="control-label" for={ snake_case(label.clone()) }>{ label }</label>
    //         <input
    //             class="form-control"
    //             type="date"
    //             id={ snake_case(label.clone()) }
    //             name={ snake_case(label.clone()) }
    //             required={props.required.clone().unwrap_or_default()}
    //             value={props.value.clone()}
    //             oninput={props.oninput.clone()}
    //         />
    //     </div>
    // }
    match &props.state {
        Some(s) => {
            html! {
                <div class="form-group ">
                    <label class="control-label" for={ snake_case(label.clone()) }>{ label }</label>
                    <input
                    class="form-control"
                    id={ snake_case(label.clone()) }
                    name={ props.name.clone().unwrap_or_else(|| snake_case(label.clone())) }
                    type="date"
                    value={(&**s).clone()}
                    required={props.required.unwrap_or_default()}
                    pattern={props.pattern.clone()}
                    {oninput}
                />
                    </div>

            }
        }
        None => html! {
        <div class="form-group ">
        <label class="control-label" for={ snake_case(label.clone()) }>{ label }</label>
            <input
                class="form-control"
                id={ snake_case(label.clone()) }
                name={ props.name.clone().unwrap_or_else(|| snake_case(label.clone())) }
                type="date"
                value={props.value.clone()}
                required={props.required.unwrap_or_default()}
                pattern={props.pattern.clone()}
                oninput={props.oninput.clone()}
            />
        </div>
        },
    }
}

#[function_component(PasswordField)]
pub fn password_field(props: &LabelProps) -> Html {
    let label = &props.label;

    html! {
        <div class="form-group required">
        <label class="control-label" for={ snake_case(label.clone()) }>{ label }</label>
            <input
                class="form-control"
                id={ snake_case(label.clone()) }
                name={ snake_case(label.clone()) }
                required=true
                type="password"
                value={props.value.clone()}
                oninput={props.oninput.clone()}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CheckboxProps {
    pub label: String,
    pub onchange: Option<Callback<Event>>,
    pub checked: Option<bool>,
}

#[function_component(CheckboxField)]
pub fn checkbox_field(props: &CheckboxProps) -> Html {
    let label = &props.label;

    html! {
        <div class="checkbox">
            <label>
                <input
                    id={ snake_case(label.clone()) }
                    name={ snake_case(label.clone()) }
                    type="checkbox"
                    checked={props.checked.unwrap_or_default()}
                    onchange={props.onchange.clone()}
                />
                { format!(" {}", label) }
            </label>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct MultiSelectFieldProps {
    pub label: String,
    pub children: ChildrenWithProps<MultiSelectOption>,
    pub onchange: Option<Callback<Event>>,
}

#[function_component(MultiSelectField)]
pub fn multi_select_field(props: &MultiSelectFieldProps) -> Html {
    let label = &props.label;

    html! {
        <div class="form-group  required">
            <label class="control-label" for={ snake_case(label.clone()) }>{ label.clone() }</label>
            <select
                class="form-control"
                id={ snake_case(label.clone()) }
                multiple=true
                name={ snake_case(label.clone()) }
                required=true
                onchange={props.onchange.clone()}
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
