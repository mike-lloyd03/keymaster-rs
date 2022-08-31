use crate::routes::Route;

use crate::types::SessionInfo;
use yew::prelude::*;
use yew_hooks::use_click_away;
use yew_router::prelude::Link;
use yewdux::prelude::*;

#[derive(PartialEq, Clone)]
pub struct LinkItem {
    pub label: String,
    pub route: Route,
}

#[function_component(Nav)]
pub fn nav() -> Html {
    let (user, _) = use_store::<SessionInfo>();

    let links = vec![
        LinkItem {
            label: "AssignKey".into(),
            route: Route::AssignKey,
        },
        LinkItem {
            label: "Assignments".into(),
            route: Route::Assignments,
        },
        LinkItem {
            label: "Keys".into(),
            route: Route::Keys,
        },
        LinkItem {
            label: "Users".into(),
            route: Route::Users,
        },
    ];

    let auth_links = links
        .iter()
        .map(|l| {
            html! {
                <NavLink route={l.route.clone()} label={l.label.clone()}/>
            }
        })
        .collect();

    html! {
        <nav class="bg-gray-800">
            <div class="relative mx-auto flex flex-row items-center h-16">
                <div class="flex-1">
                    <MobileMenu {links} />
                    <div class="hidden sm:block">
                        <Logo label="KeyMaster" />
                        {
                            if user.is_auth {
                                auth_links
                            } else {
                                html!{}
                            }
                        }
                    </div>
                </div>
                <div class="absolute right-0 pr-2">
                    {
                        if user.is_auth {
                            html!{
                                <NavDropdown label={user.username.clone().unwrap_or_default()}>
                                    <NavDropdownItem label="Set Password" route={Route::SetPassword { username: user.username.clone().unwrap_or_default() }} />
                                    <NavDropdownItem label="Logout" route={Route::Logout} />
                                </NavDropdown>
                            }
                        } else {
                            html! {
                                <Link<Route> to={Route::Login} classes="text-gray-300">{ "Login" }</Link<Route>>
                            }
                        }
                    }
                </div>
            </div>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub struct LogoProps {
    label: String,
}

#[function_component(Logo)]
pub fn logo(props: &LogoProps) -> Html {
    let classes = classes!(
        "text-blue-500",
        "hover:text-white",
        "px-3",
        "py-2",
        "rounded-md",
        "text-lg",
        "font-medium"
    );
    html! {
        <Link<Route> to={Route::Home} {classes}>{ props.label.clone() }</Link<Route>>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavLinkProps {
    route: Route,
    label: String,
}

#[function_component(NavLink)]
pub fn nav_link(props: &NavLinkProps) -> Html {
    let classes = classes!(
        "text-gray-300",
        "hover:text-white",
        "hover:bg-gray-700",
        "px-3",
        "py-2",
        "rounded-md",
        "text-sm",
        "font-medium"
    );
    html! {
            <Link<Route> to={props.route.clone()} {classes}>{ props.label.clone() }</Link<Route>>
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

    let menu_classes_hidden = classes!(
        "absolute",
        "right-0",
        "mt-2",
        "w-48",
        "origin-top-right",
        "rounded-md",
        "bg-white",
        "py-1",
        "shadow-lg",
        "ring-1",
        "ring-black",
        "ring-opacity-5",
        "focus:outline-none",
        "hidden"
    );
    let menu_classes_shown = classes!(
        "absolute",
        "right-0",
        "mt-2",
        "w-48",
        "origin-top-right",
        "rounded-md",
        "bg-white",
        "py-1",
        "shadow-lg",
        "ring-1",
        "ring-black",
        "ring-opacity-5",
        "focus:outline-none"
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
        <div
            class={"relative ml-3"}
            ref={node}
        >
            <button
                class="flex text-cyan-300 hover:text-white hover:bg-gray-700 px-3 py-2 rounded-md font-medium"
                data-toggle="dropdown"
                {onclick}
            >
                {props.label.clone()}
            </button>
            <div
                class={
                    match *show {
                        true => menu_classes_shown,
                        false => menu_classes_hidden
                    }
                }
            >
            { for props.children.clone() }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavDropdownItemProps {
    label: String,
    route: Route,
}

#[function_component(NavDropdownItem)]
pub fn nav_dropdown_item(props: &NavDropdownItemProps) -> Html {
    let classes = "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-200";
    html! {
        <Link<Route> {classes} to={props.route.clone()}>{props.label.clone()}</Link<Route>>
    }
}

#[derive(Properties, PartialEq)]
pub struct MobileMenuProps {
    pub links: Vec<LinkItem>,
}

#[function_component(MobileMenu)]
pub fn mobile_menu(props: &MobileMenuProps) -> Html {
    let show = use_state(|| false);
    let node = use_node_ref();

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
        <>
            <div class="absolute inset-y-0 left-0 flex items-center sm:hidden">
                <button
                    type="button"
                    class="inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white"
                    {onclick}
                >
                  <svg
                      class="block h-6 w-6"
                      xmlns="http://www.w3.org/2000/svg"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke-width="1.5"
                      stroke="currentColor"
                  >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
                    />
                  </svg>
                  <svg
                      class="hidden h-6 w-6"
                      xmlns="http://www.w3.org/2000/svg"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke-width="1.5"
                      stroke="currentColor"
                  >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M6 18L18 6M6 6l12 12"
                    />
                  </svg>
                </button>
              </div>
          {
              match *show {
                true => html!{<MobileMenuDropdown links={props.links.clone()} node_ref={node}/>},
                false => html!{}
            }
          }
      </>
    }
}

#[derive(Properties, PartialEq)]
pub struct MobileMenuDropdownProps {
    links: Vec<LinkItem>,
    node_ref: NodeRef,
}

#[function_component(MobileMenuDropdown)]
pub fn mobile_menu_dropdown(props: &MobileMenuDropdownProps) -> Html {
    let mobile_links = props.links.iter().map(|l| {
        html! {
            <MobileMenuItem route={l.route.clone()} label={l.label.clone()} />
        }
    });

    html! {
    <div class="sm:hidden absolute top-16 bg-gray-800" id="mobile-menu" ref={props.node_ref.clone()}>
        <div class="space-y-1 px-2 pt-2 pb-3">
            <MobileMenuItem route={Route::Home} label="Home" />
            { for mobile_links }
        </div>
      </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct MobileMenuItemProps {
    label: String,
    route: Route,
}

#[function_component(MobileMenuItem)]
pub fn mobile_menu_item(props: &MobileMenuItemProps) -> Html {
    let classes = classes!(
        "text-gray-300",
        "hover:bg-gray-700",
        "hover:text-white",
        "block",
        "px-3",
        "py-2",
        "rounded-md",
        "text-base",
        "font-medium"
    );
    html! {
          <Link<Route> {classes} to={props.route.clone()}>{props.label.clone()}</Link<Route>>
    }
}
