use crate::theme::*;
use crate::{routes::Route, services::auth::current_user};
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub heading: String,
    pub value: Option<String>,
    pub edit_route: Option<Route>,
}

#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
    let button_enabled_classes = classes!("font-medium", TEXT_BLUE, "hover:underline");
    let cl_edit_btn = if current_user().is_admin {
        button_enabled_classes
    } else {
        classes!(button_enabled_classes, "disabled")
    };

    let td_classes = classes!("py-4", "px-6");

    match props.edit_route.clone() {
        Some(route) => html! {
            <td class={td_classes}>
                <Link<Route> classes={cl_edit_btn} to={route}>
                    {"Edit"}
                </Link<Route>>
            </td>
        },
        None => html! {
            <td class={td_classes}>{ props.value.clone().unwrap_or_default() }</td>
        },
    }
}

#[derive(Properties, PartialEq)]
pub struct RowProps {
    pub children: ChildrenWithProps<Cell>,
}

#[function_component(Row)]
pub fn row(props: &RowProps) -> Html {
    let tr_classes = classes!(
        "border-y",
        BG_SEC_DARK,
        "border-gray-700",
        "hover:bg-gray-700",
        "hover:border-gray-800"
    );
    html! {
        <tr class={tr_classes}>{ for props.children.iter() }</tr>
    }
}
#[derive(Properties, PartialEq)]
pub struct TableCaptionProps {
    pub title: String,
}

#[function_component(TableCaption)]
pub fn table_caption(props: &TableCaptionProps) -> Html {
    html! {
        <caption class="p-5 text-lg font-semibold text-left text-gray-900 bg-white dark:text-white dark:bg-gray-800">
        {props.title.clone()}
            // <p class="mt-1 text-sm font-normal text-gray-500 dark:text-gray-400">Browse a list of Flowbite products designed to help you work and play, stay organized, get answers, keep in touch, grow your business, and more.</p>
        </caption>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableProps {
    pub title: String,
    pub button_label: Option<String>,
    pub button_route: Option<Route>,
    pub headings: Option<Vec<&'static str>>,
    pub children: ChildrenWithProps<Row>,
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let cl_action_btn = classes!(BTN, BTN_PRIMARY);

    let cl_table_container = classes!(
        BG_PRIME_DARK,
        TEXT_DARK,
        "rounded-xl",
        "relative",
        "overflow-x-auto",
        "shadow-md",
        "text-center",
        "py-1",
    );

    let cl_title = classes!("p-5", "text-lg", "font-semibold", "text-left", "text-white",);

    let cl_table = classes!("w-full", "text-sm", "text-center", TEXT_GRAY,);

    let cl_table_headings = classes!(
        "text-xs",
        "uppercase",
        "bg-gray-700",
        "text-gray-400"
    );

    html! {

        <div class={cl_table_container}>
            // Header section
            <div class="grid grid-flow-col auto-cols-auto items-center">
                <div class={cl_title}>
                    {props.title.clone()}
                </div>
                {
                    match props.button_label.clone() {
                        Some(label) => {
                            let route = props.button_route.clone().unwrap_or(Route::Home);
                            html! {
                                <div class="text-right"> <Link<Route> classes={cl_action_btn} to={route}>
                                    {label}
                                </Link<Route>> </div>
                            }
                        }
                        None => html!{},
                    }
                }
            </div>

                // Table Section
            <div class="overflow-x-auto">
                <table class={cl_table}>
                    <thead class={cl_table_headings}>
                        <tr>
                            {
                                match props.headings.clone() {
                                    Some(h) => h.iter().map(|h|
                                                            html!{<TableHeading label={h.clone()} />
                                                            }).collect(),
                                    None => html!{},
                                }
                            }
                        </tr>
                    </thead>
                    <tbody>
                        { for props.children.iter()}
                    </tbody>
                </table>
            </div>
        </div>

    }
}

#[derive(Properties, PartialEq)]
pub struct TableHeadingProps {
    pub label: String,
}

#[function_component(TableHeading)]
pub fn table_heading(props: &TableHeadingProps) -> Html {
    let th_classes = classes!("py-3", "px-6");
    html! {
        <th class={th_classes}>{props.label.clone()}</th>
    }
}
