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
    let button_enabled_classes = classes!(
        "font-medium",
        "text-blue-600",
        "dark:text-blue-500",
        "hover:underline"
    );
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
        "bg-white",
        "border-b",
        "dark:bg-gray-800",
        "dark:border-gray-700",
        "hover:bg-gray-50",
        "dark:hover:bg-gray-600"
    );
    html! {
        <tr class={tr_classes}>{ for props.children.iter() }</tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableProps {
    pub title: String,
    pub button_label: Option<String>,
    pub button_route: Option<Route>,
    pub children: ChildrenWithProps<Row>,
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let headings: Vec<String> = match props.children.iter().next() {
        Some(c) => c
            .props
            .children
            .iter()
            .map(|cell| cell.props.heading.clone())
            .collect(),
        None => vec!["".to_string()],
    };

    let cl_action_btn = classes!(BTN, BTN_PRIMARY);

    let cl_table_bg = classes!(
        BG_DARK,
        TEXT_DARK,
        "rounded-xl",
        "relative",
        "overflow-x-auto",
        "shadow-md",
        "text-center",
        "py-1",
    );

    let cl_table = classes!("w-full", "text-sm", "text-left", "text-gray-400",);

    html! {

        <div class={cl_table_bg}>
            <h2>{ props.title.clone() }</h2>
            {
                match props.button_label.clone() {
                    Some(label) => {
                        let route = props.button_route.clone().unwrap_or(Route::Home);
                        html! {
                            <div class="">
                                <Link<Route> classes={cl_action_btn} to={route}>
                                {label}
                            </Link<Route>>
                                </div>
                        }
                    }
                    None => html!{},
                }
            }
        <table class={cl_table}>
            <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
            <tr>
            { for headings.iter().map(|heading| html!{<TableHeading label={heading.clone()} />}) }
        </tr>
            </thead>
            <tbody>
                { for props.children.iter()}
            </tbody>
            </table>
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
