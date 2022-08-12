use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FormProps {
    pub title: String,
    pub subtitle: Option<String>,
    pub action_label: String,
    pub children: Children,
}

pub struct Form;

impl Component for Form {
    type Message = ();
    type Properties = FormProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container text-light my-3" style="max-width: 600px;">
                <div class="row justify-content-center">
                    <h1>{ ctx.props().title.clone() }</h1>
                    {
                        match ctx.props().subtitle.clone() {
                            Some(s) => html!{<h4>{ s.to_string() }</h4>},
                            None => html!{{ "" }},
                        }
                    }
                    {
                        for ctx.props().children.iter()
                    }
                    <form action="" method="post" class="form" role="form">
                        <input
                            class="btn btn-primary"
                            id="submit"
                            name="submit"
                            type="submit"
                            value={ ctx.props().action_label.clone() } />
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
                name={ snake_case(label.clone()) }
                type="text"
                value=""
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LabelProps {
    pub label: String,
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
    // pub children: ChildrenWithProps<MultiSelectOption>,
    pub children: Children,
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

fn snake_case(s: String) -> String {
    str::replace(&s.to_lowercase(), " ", "_")
}
