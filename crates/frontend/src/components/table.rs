use crate::theme::*;
use crate::{routes::Route, services::auth::current_user};
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct TableCardProps {
    pub title: String,
    pub button_label: Option<String>,
    pub button_route: Option<Route>,
    pub headings: Option<Vec<&'static str>>,
    pub children: ChildrenWithProps<Row>,
}

#[function_component(TableCard)]
pub fn table_card(props: &TableCardProps) -> Html {
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

    html! {
        <div class={cl_table_container}>

            <TableHeader title={props.title.clone()}>
                {
                    if current_user().is_admin {
                        match props.button_label.clone() {
                            Some(label) => {
                                let route = props.button_route.clone().unwrap_or(Route::Home);
                                html! {
                                    <ActionButton {label} {route} />
                                }
                            }
                            None => html!{},
                        }
                    } else {html!{}}
                }
            </TableHeader>

            <Table headings={props.headings.clone()}>
                { for props.children.iter()}
            </Table>
        </div>

    }
}

#[derive(Properties, PartialEq)]
pub struct TableHeaderProps {
    pub title: String,
    pub children: Children,
}

#[function_component(TableHeader)]
pub fn table_header(props: &TableHeaderProps) -> Html {
    let cl_title = classes!("p-5", "text-lg", "font-semibold", "text-left", "text-white",);

    html! {
        <div class="grid grid-flow-col auto-cols-auto items-center">
            <div class={cl_title}>
                {props.title.clone()}
            </div>
                {for props.children.iter()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TableProps {
    pub headings: Option<Vec<&'static str>>,
    pub children: ChildrenWithProps<Row>,
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let cl_table = classes!("w-full", "text-sm", "text-left", TEXT_GRAY,);

    let cl_table_headings = classes!("text-xs", "uppercase", "bg-gray-700", "text-gray-400");

    html! {
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
    }
}

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub value: Option<String>,
}

#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
    let td_classes = classes!("py-4", "px-6");

    html! {
        <td class={td_classes}>{ props.value.clone().unwrap_or_default() }</td>
    }
}

#[derive(Properties, PartialEq)]
pub struct CellLinkProps {
    pub value: String,
    pub route: Route,
}

#[function_component(CellLink)]
pub fn cell_link(props: &CellLinkProps) -> Html {
    let td_classes = classes!("py-4", "px-6");

    html! {
        <td class={td_classes}>
            <Link<Route> classes={"hover:underline"} to={props.route.clone()}>
                {props.value.clone()}
            </Link<Route>>
        </td>
    }
}

#[derive(Properties, PartialEq)]
pub struct CellEditProps {
    pub route: Route,
}

#[function_component(CellEdit)]
pub fn cell_edit(props: &CellEditProps) -> Html {
    let cl_button = classes!("font-medium", TEXT_BLUE, "hover:underline");
    let cl_edit_btn = if current_user().is_admin {
        cl_button
    } else {
        classes!(
            cl_button,
            "text-gray-500",
            "opacity-60",
            "pointer-events-none"
        )
    };

    let td_classes = classes!("py-4", "px-6");

    html! {
        <td class={td_classes}>
            <Link<Route> classes={cl_edit_btn} to={props.route.clone()}>
                {"Edit"}
            </Link<Route>>
        </td>
    }
}

#[derive(Properties, PartialEq)]
pub struct RowProps {
    pub children: Children,
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

#[derive(Properties, PartialEq)]
pub struct ActionButtonProps {
    pub label: String,
    pub route: Route,
}

#[function_component(ActionButton)]
pub fn action_button(props: &ActionButtonProps) -> Html {
    let cl_action_btn = classes!(BTN, BTN_PRIMARY);

    html! {
        <div class="text-right">
            <Link<Route> classes={cl_action_btn} to={props.route.clone()}>
                {&props.label}
            </Link<Route>>
        </div>
    }
}
