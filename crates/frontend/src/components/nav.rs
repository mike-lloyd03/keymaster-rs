use crate::routes::Route;

use crate::types::SessionInfo;
use yew::prelude::*;
use yew_hooks::use_click_away;
use yew_router::prelude::Link;
use yewdux::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    let (user, _) = use_store::<SessionInfo>();
    let auth_links = html! {
        <>
            <li class="nav-item">
                <Link<Route> to={Route::AssignKey} classes="nav-link">{ "Assign Key" }</Link<Route>>
            </li>
            <li>
                <Link<Route> to={Route::Assignments} classes="nav-link">{ "Assignments" }</Link<Route>>
            </li>
            <li>
                <Link<Route> to={Route::Keys} classes="nav-link">{ "Keys" }</Link<Route>>
            </li>
            <li>
                <Link<Route> to={Route::Users} classes="nav-link">{ "Users" }</Link<Route>>
            </li>

        </>
    };

    html! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
            <div class="container-fluid">
                <Link<Route> to={Route::Home} classes={classes!("navbar-brand", "text-primary")}>{ "KeyMaster" }</Link<Route>>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                        {
                            if user.is_auth {
                                auth_links
                            } else {
                                html!{}
                            }
                        }
                    </ul>
                        {
                            if user.is_auth {
                                html!{
                                    <>
                                        <NavDropdown label={user.username.clone().unwrap_or_default()}>
                                            <NavDropdownItem label="Logout" route={Route::Logout} />
                                        </NavDropdown>
                                    </>
                                }
                            } else {
                                html! {
                                    <div class="nav-item">
                                        <Link<Route> to={Route::Login} classes="nav-link">{ "Login" }</Link<Route>>
                                    </div>
                                }
                            }
                        }
                </div>
            </div>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavDropdownProps {
    label: String,
    children: ChildrenWithProps<NavDropdownItem>,
}

#[function_component(NavDropdown)]
pub fn nav_dropdown(props: &NavDropdownProps) -> Html {
    let show = use_state(|| false);
    let node = use_node_ref();

    let item_classes_hidden = classes!("nav-item", "dropdown");
    let item_classes_shown = classes!("nav-item", "dropdown", "show");
    let menu_classes_hidden = classes!("dropdown-menu", "bg-dark", "text-light", "user-dropdown");
    let menu_classes_shown = classes!(
        "dropdown-menu",
        "bg-dark",
        "text-light",
        "user-dropdown",
        "show"
    );

    let onclick = {
        let show = show.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            show.set(!*show);
        })
    };

    {
        let show = show.clone();
        use_click_away(node.clone(), move |_: Event| {
            show.set(false);
        });
    }

    html! {
        <li
            class={
                match *show {
                    true => item_classes_shown,
                    false => item_classes_hidden,
                }
            }
            ref={node}
        >
            <div
                class="nav-link dropdown-toggle"
                role="button"
                data-toggle="dropdown"
                {onclick}
            >
                {props.label.clone()}
            </div>
            <div
                class={
                    match *show {
                        true => menu_classes_shown,
                        false => menu_classes_hidden,
                    }
                }
            >
            { for props.children.clone() }
            </div>
        </li>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavDropdownItemProps {
    label: String,
    route: Route,
}

#[function_component(NavDropdownItem)]
pub fn nav_dropdown_item(props: &NavDropdownItemProps) -> Html {
    let classes = classes!("dropdown-item", "bg-dark", "text-muted",);
    html! {
        <Link<Route> {classes} to={props.route.clone()}>{props.label.clone()}</Link<Route>>
    }
}
