use std::fmt::Display;

use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew_router::prelude::{use_history, History};

use crate::components::notifier::notify_warn;
use crate::routes::Route;
use crate::theme::*;

#[derive(Properties, PartialEq)]
pub struct FormProps {
    pub title: String,
    pub subtitle: Option<String>,
    pub onsubmit: Option<Callback<FocusEvent>>,
    pub children: Children,
}

#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    let cl_form_container = classes!(
        "p-4",
        "w-full",
        "max-w-md",
        "mx-auto",
        "rounded-lg",
        "border",
        "shadow-md",
        "sm:p-6",
        "md:p-8",
        "bg-gray-800",
        "border-gray-700"
    );

    html! {
    <div class={cl_form_container}>
                <h5 class="text-xl font-medium text-white">
                    { props.title.clone() }
                </h5>
                {
                    match props.subtitle.clone() {
                        Some(s) => html!{
                            <h6 class="text-lg font-medium my-3 text-gray-300">
                                {s.to_string()}
                            </h6>},
                        None => html!{},
                    }
                }
                <form
                    action="#"
                    method="post"
                    id="form"
                    class="space-y-6"
                    role="form"
                    onsubmit={ props.onsubmit.clone() }
                >
                    {
                        for props.children.iter()
                    }
                </form>
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
                    ButtonType::Primary => classes!(BTN, BTN_PRIMARY),
                    ButtonType::Secondary => classes!(BTN, BTN_SECONDARY),
                    ButtonType::Danger => classes!(BTN, BTN_DANGER),
                }
            }
            type="submit"
            value={ props.value.clone() }
            onclick={ props.onclick.clone() }
        />
    }
}

#[derive(Properties, PartialEq)]
pub struct RouteButtonProps {
    pub value: String,
    pub route: Route,
}

#[function_component(RouteButton)]
pub fn route_button(props: &RouteButtonProps) -> Html {
    let oncancel = {
        let history = use_history().unwrap();
        let route = props.route.clone();
        Callback::once(move |_: MouseEvent| history.push(route))
    };

    html! {
        <Button value={props.value.clone()} button_type={ButtonType::Secondary} onclick={oncancel} />
    }
}

#[derive(Properties, PartialEq)]
pub struct DeleteButtonProps {
    pub value: String,
    pub route: Route,
    pub show_modal: UseStateHandle<bool>,
}

#[function_component(DeleteButton)]
pub fn delete_button(props: &DeleteButtonProps) -> Html {
    let onclick_delete = {
        let show_modal = props.show_modal.clone();
        Callback::once(move |e: MouseEvent| {
            e.prevent_default();
            show_modal.set(true);
        })
    };

    html! {
        <Button value={props.value.clone()} button_type={ButtonType::Danger} onclick={onclick_delete} />
    }
}

#[derive(Properties, PartialEq)]
pub struct SortButtonProps {
    pub value: String,
    pub route: Route,
}

#[function_component(SortButton)]
pub fn sort_button(props: &SortButtonProps) -> Html {
    let oncancel = {
        let history = use_history().unwrap();
        let route = props.route.clone();
        Callback::once(move |_: MouseEvent| history.push(route))
    };

    html! {
        <Button value="Cancel" button_type={ButtonType::Secondary} onclick={oncancel} />
    }
}

#[derive(Properties, PartialEq)]
pub struct LabelProps {
    pub value: String,
    pub for_input: String,
}

#[function_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    let cl_label = classes!("block", "mb-2", "text-sm", "font-medium", "text-gray-300");

    html! {
        <label class={cl_label} for={props.for_input.clone()}>{props.value.clone()}</label>
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
    pub oninvalid: Option<Callback<Event>>,
    pub required: Option<bool>,
    pub pattern: Option<String>,
    pub checked: Option<bool>,
}

#[function_component(InputField)]
pub fn input_field(props: &InputProps) -> Html {
    let label_snake = snake_case(props.label.clone());

    let cl_input = classes!(
        "border",
        "text-sm",
        "rounded-lg",
        "focus:ring-blue-500",
        "focus:border-blue-500",
        "block",
        "w-full",
        "p-2.5",
        "bg-gray-600",
        "border-gray-500",
        "placeholder-gray-400",
        "text-white"
    );

    let oninput = {
        let state = props.state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            state.set(input.value());
        })
    };

    html! {
        <div class="form-group ">
            <Label value={props.label.clone()} for_input={label_snake.clone()} />
            <input
                class={cl_input}
                name={ label_snake.clone() }
                type={ props.input_type.clone().unwrap().to_string() }
                value={(&*props.state).clone()}
                required={props.required.unwrap_or_default()}
                pattern={props.pattern.clone()}
                {oninput}
                oninvalid={props.oninvalid.clone()}
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
    let oninvalid = Callback::from(move |_: Event| {
        notify_warn("The password must be at least 8 characters long");
    });

    html! {
        <InputField
            input_type={InputType::Password}
            label={props.label.clone()}
            state={props.state.clone()}
            name={props.name.clone()}
            value={props.value.clone()}
            oninput={props.oninput.clone()}
            required={props.required}
            pattern="(.){8,}"
            {oninvalid}
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

    // let cl_input = classes!(
    //     "w-4",
    //     "h-4",
    //     "rounded",
    //     "border",
    //     "bg-gray-700",
    //     "border-gray-600",
    //     "focus:ring-blue-600",
    //     "ring-offset-gray-800",
    //     "checked:bg-blue-700",
    //     "checked:border-blue-700",
    //     "cursor-pointer"
    // );

    html! {
        <div class="flex items-start">
            <div class="flex items-center h-5">
                <input
                    // style="appearance: none;"
                    name={snake_case(label.clone())}
                    type="checkbox"
                    checked={*(props.state)}
                    // class={cl_input}
                    {onchange}
                />
            </div>
            <Label value={format!(" {}", label)} for_input={snake_case(label.clone())} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct MultiSelectFieldProps {
    pub label: String,
    pub state: UseStateHandle<Vec<String>>,
    pub children: ChildrenWithProps<MultiSelectOption>,
}

// <label for="countries" class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-400">Select your country</label>
// <select id="countries" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
//   <option>United States</option>
//   <option>Canada</option>
//   <option>France</option>
//   <option>Germany</option>
// </select>

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

    let cl_input = classes!(
        "border",
        "text-sm",
        "rounded-lg",
        "block",
        "w-full",
        "p-2.5",
        "bg-gray-700",
        "border-gray-600",
        "placeholder-gray-400",
        "text-white",
        "focus:ring-blue-500",
        "focus:border-blue-500"
    );

    html! {
        <div class="form-group  required">
            <Label for_input={label_sn.clone()} value={ props.label.clone() } />
            <select
                class={cl_input}
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
