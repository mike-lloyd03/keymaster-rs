use yew::prelude::*;

// Lets try to implement a trait
// FormField should be able to handle different types of Field objects:
// - Text
// - Checkbox
// - Multi-select
// - Date
//
// They should all implement a to_field function to generate the html.
// The form object should be able to accept an arbitrary length array of form fields and render
// them in a list.

fn capitalize(mut s: String) -> String {
    s.remove(0).to_uppercase().to_string() + &s
}

#[derive(Properties, PartialEq)]
pub struct FormProps {
    pub title: String,
    pub subtitle: Option<String>,
    pub item: String,
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
            <div class="">
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
                        value={ format!("Add {}", ctx.props().item) } />
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
        }
    }
}

#[function_component(TextField)]
pub fn form_text_field(props: &FormTextFieldProps) -> Html {
    html! {
        <div class="form-group "><label class="control-label" for={ props.label.clone() }>
            { capitalize(props.label.clone()) }
        </label>
            <input class="form-control" id={ props.label.clone() } name={ props.label.clone() } type="text" value="" />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct FormTextFieldProps {
    pub label: String,
}

#[function_component(CheckboxField)]
pub fn form_checkbox_field(props: &FormCheckboxFieldProps) -> Html {
    html! {
        <div class="checkbox">
            <label>
                <input
                    id={ props.label.clone() }
                    name={ props.label.clone() }
                    type="checkbox"
                    value="n"
                /> { capitalize(props.label.clone() )}
            </label>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct FormCheckboxFieldProps {
    pub label: String,
}
