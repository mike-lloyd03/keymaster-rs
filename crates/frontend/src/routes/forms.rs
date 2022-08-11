use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FormFieldProps {
    field_name: String,
}

#[function_component(FormField)]
pub fn form_field(props: &FormFieldProps) -> Html {
    html! {
        <div class="form-group "><label class="control-label" for={ props.field_name.clone() }>
            { capitalize(props.field_name.clone()) }
        </label>
            <input class="form-control" id={ props.field_name.clone() } name={ props.field_name.clone() } type="text" value="" />
        </div>
    }
}

fn capitalize(mut s: String) -> String {
    s.remove(0).to_uppercase().to_string() + &s
}

#[derive(Properties, PartialEq)]
pub struct FormProps {
    pub item: String,
    pub fields: Vec<&'static str>,
}

#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    let fields: Html = props
        .fields
        .iter()
        .map(|field| html! { <FormField field_name= { field.to_string() } />})
        .collect();

    html! {
        <div class="">
            <h1>{ format!("New {}", props.item) }</h1>
            <form action="" method="post" class="form" role="form">
                { fields }
                <input
                    class="btn btn-primary"
                    id="submit"
                    name="submit"
                    type="submit"
                    value={ format!("Add {}", props.item) } />
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
