use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FormProps {
    pub title: String,
    pub subtitle: Option<String>,
    pub action: Option<String>,
    pub method: Option<String>,
    pub submit_label: String,
    pub redirect: Option<String>,
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
                        None => html!{{ "" }},
                    }
                }
                <form action={
                        format!("http://localhost:8080/api/{}",
                        props.action.clone().unwrap_or_else(|| "".to_string())
                        )
                    }
                    method="post"
                    class="form"
                    role="form"
                    onsubmit={ props.onsubmit.clone() }
                >
                    {
                        for props.children.iter()
                    }
                    <input
                        class="btn btn-primary"
                        id="submit"
                        name="submit"
                        type="submit"
                        value={ props.submit_label.clone() }
                    />
                    {" "}
                    <input
                        class="btn btn-secondary"
                        formnovalidate=true
                        id="cancel"
                        name="cancel"
                        type="submit"
                        value="Cancel" />
                </form>
            </div>
        </div>
    }
}

#[function_component(TextField)]
pub fn text_field(props: &LabelProps) -> Html {
    let label = &props.label;

    html! {
        <div class="form-group ">
        <label class="control-label" for={ snake_case(label.clone()) }>{ label }</label>
            <input
                class="form-control"
                id={ snake_case(label.clone()) }
                name={ props.name.clone().unwrap_or_else(|| snake_case(label.clone())) }
                type="text"
                value={props.value.clone()}
                onchange={props.onchange.clone()}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LabelProps {
    pub label: String,
    pub name: Option<String>,
    pub value: Option<String>,
    pub onchange: Option<Callback<Event>>,
}

#[function_component(CheckboxField)]
pub fn checkbox_field(props: &LabelProps) -> Html {
    let label = &props.label;

    html! {
        <div class="checkbox">
            <label>
                <input
                    id={ snake_case(label.clone()) }
                    name={ snake_case(label.clone()) }
                    type="checkbox"
                    value="n"
                />{ format!(" {}", label) }
            </label>
        </div>
    }
}

#[function_component(MultiSelectField)]
pub fn multi_select_field(props: &MultiSelectFieldProps) -> Html {
    let label = &props.label;

    html! {
        <div class="form-group  required">
            <label class="control-label" for={ snake_case(label.clone()) }>{ label.clone() }</label>
            <select class="form-control" id="user" multiple=true name={ snake_case(label.clone()) } required=true>
                {
                    for props.children.iter()
                }
            </select>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct MultiSelectFieldProps {
    pub label: String,
    pub children: ChildrenWithProps<MultiSelectOption>,
}

#[function_component(MultiSelectOption)]
pub fn multi_select_option(props: &MultiSelectOptionProps) -> Html {
    html! {
        <option value={ props.value.clone() }>{
            match props.label.clone() {
                Some(l) => l,
                None => props.value.clone()
            }
        }</option>
    }
}

#[derive(Properties, PartialEq)]
pub struct MultiSelectOptionProps {
    pub value: String,
    pub label: Option<String>,
}

#[function_component(DateField)]
pub fn date_field(props: &LabelProps) -> Html {
    let label = &props.label;

    html! {
        <div class="form-group required">
            <label class="control-label" for={ snake_case(label.clone()) }>{ label }</label>
            <input
                class="form-control"
                id={ snake_case(label.clone()) }
                name={ snake_case(label.clone()) }
                required=true
                type="date"
                value=""
            />
        </div>
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
                value=""
            />
        </div>
    }
}

fn snake_case(s: String) -> String {
    str::replace(&s.to_lowercase(), " ", "_")
}
