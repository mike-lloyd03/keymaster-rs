use crate::components::table::ActionButton;
use crate::routes::Route;
use crate::services::auth::current_user;
use crate::theme::*;
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct DetailsCardProps {
    pub title: String,
    pub edit_route: Route,
    pub children: Children,
}

#[function_component(DetailsCard)]
pub fn details_card(props: &DetailsCardProps) -> Html {
    html! {
        <div class="container mx-auto max-w-lg my-5">
            <div>
                <div class="grid grid-flow-col auto-cols-auto items-center">
                    <div class={DETAIL_CARD}>{props.title.clone()}
                        {
                            if current_user().is_admin {
                                html!{
                                    <ActionButton label={"Edit"} route={props.edit_route.clone()} />
                                }
                            } else {
                                html!{}
                            }
                        }
                    </div>
                </div>
            </div>
            {for props.children.iter()}
        </div>
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct DetailsHeaderItemProps {
    pub content: String,
}

#[function_component(DetailsHeaderItem)]
pub fn details_header_item(props: &DetailsHeaderItemProps) -> Html {
    html! {
        <p class={DETAIL_HEADER_ITEM}>{props.content.clone()}</p>
    }
}

#[derive(Properties, PartialEq)]
pub struct DetailsHeaderProps {
    pub children: Children,
}

#[function_component(DetailsHeader)]
pub fn details_header(props: &DetailsHeaderProps) -> Html {
    html! {
      <div>
        <div class={DETAIL_HEADER}>
            {for props.children.iter()}
        </div>
      </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DetailsListProps {
    pub label: String,
    pub children: ChildrenWithProps<DetailsListItem>,
}

#[function_component(DetailsList)]
pub fn details_list(props: &DetailsListProps) -> Html {
    html! {
        <>
            <div>
                <span class={DETAIL_LIST}>{props.label.clone()}</span>
            </div>
            <div class="">
                <div class={DETAIL_LIST_CONTAINER}>
                    <ul role="list" class="divide-y divide-gray-700">
                        {for props.children.iter()}
                    </ul>
                </div>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct DetailsListItemProps {
    pub label: String,
    pub route: Route,
}

#[function_component(DetailsListItem)]
pub fn details_list_item(props: &DetailsListItemProps) -> Html {
    html! {
        <li class={DETAIL_LIST_ITEM_ROW}>
            <Link<Route> to={props.route.clone()} classes={DETAIL_LIST_ITEM_LINK}>
                {format!("- {}", props.label.clone())}
            </Link<Route>>
        </li>
    }
}

#[function_component(DetailsFooter)]
pub fn details_footer() -> Html {
    html! {
        <div class={DETAIL_FOOTER}></div>
    }
}
