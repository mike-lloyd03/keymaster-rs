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
    let button_enabled_classes = classes!("btn", "btn-outline-primary");
    let button_disabled_classes = classes!("btn", "btn-outline-secondary", "disabled");
    let classes = if current_user().is_admin {
        button_enabled_classes
    } else {
        button_disabled_classes
    };

    match props.edit_route.clone() {
        Some(route) => html! {
            <td>
                <Link<Route> classes={classes} to={route}>
                    {"Edit"}
                </Link<Route>>
            </td>
        },
        None => html! {
            <td>{ props.value.clone().unwrap_or_default() }</td>
        },
    }
}

#[derive(Properties, PartialEq)]
pub struct RowProps {
    pub children: ChildrenWithProps<Cell>,
}

#[function_component(Row)]
pub fn row(props: &RowProps) -> Html {
    html! {
        <tr>{ for props.children.iter() }</tr>
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

    html! {
        <div style="text-align: center">
            <h2>{ props.title.clone() }</h2>
            {
                match props.button_label.clone() {
                    Some(label) => {
                        let route = props.button_route.clone().unwrap_or(Route::Home);
                        html! {
                            <div class="container py-2">
                                <Link<Route> classes={classes!("btn", "btn-primary")} to={route}>
                                {label}
                            </Link<Route>>
                            </div>
                        }
                    }
                    None => html!{},
                }
            }
            <table class="table table-striped table-hover table-bordered table-dark">
                <thead class="table-dark">
                    <tr>
                        { for headings.iter().map(|heading| html!{<th>{ heading.clone() }</th>}) }
                    </tr>
                </thead>
                <tbody>
                    { for props.children.iter()}
                </tbody>
            </table>
        </div>
    }
}
