use crate::{
    routes::Route,
    services::requests::get,
    types::{SessionInfo, User},
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct NavProps {
    children: Children,
}

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let (user, _) = use_store::<SessionInfo>();
    let show_mobile_menu = use_state(|| false);

    let onclick = {
        let show_mobile_menu = show_mobile_menu.clone();
        Callback::from(move |_: MouseEvent| show_mobile_menu.set(!(*show_mobile_menu)))
    };

    html! {
    <Nav>
        <MobileMenuButton {onclick} />
        <Logo />
        <NavLinks show={show_mobile_menu}>
            {
                if user.is_auth {
                    html!{
                        <>
                        {
                            if user.is_admin {
                                html!{
                                    <NavLink label="Assign Key" route={Route::AssignKey}/>
                                }
                            } else {html!{}}
                        }
                            <NavLink label="Assignments" route={Route::Assignments}/>
                            <NavLink label="Keys" route={Route::Keys}/>
                            <NavLink label="Users" route={Route::Users}/>
                        </>
                    }
                } else {
                    html!{}
                }
            }
            <div class="grow"></div>
            <UserMenu user={(*user).clone()} />
        </NavLinks>
    </Nav>
    }
}

#[function_component(Nav)]
pub fn nav(props: &NavProps) -> Html {
    let classes = classes!(
        "md:mt-2",
        "md:mx-4",
        "p-3",
        "rounded",
        "bg-gray-800",
        "border-gray-700"
    );

    html! {
        <nav class={classes}>
            <div class="container flex flex-wrap justify-between items-center mx-auto">
                {for props.children.iter()}
            </div>
        </nav>
    }
}

#[function_component(Logo)]
pub fn logo() -> Html {
    let classes = classes!(
        "text-blue-500",
        "hover:text-white",
        "px-3",
        "py-2",
        "rounded-md",
        "text-lg",
        "font-medium",
        "shrink",
        "mx-auto"
    );

    html! {
        <Link<Route> to={Route::Home} {classes}>{ "KeyMaster" }</Link<Route>>
    }
}

#[derive(Properties, PartialEq)]
pub struct MobileMenuButtonProps {
    onclick: Callback<MouseEvent>,
}

#[function_component(MobileMenuButton)]
pub fn mobile_menu_button(props: &MobileMenuButtonProps) -> Html {
    let cl_button = classes!(
        "inline-flex",
        "justify-center",
        "items-center",
        "ml-3",
        "rounded-lg",
        "md:hidden",
        "focus:outline-none",
        "focus:ring-2",
        "text-gray-400",
        "hover:text-white",
        "focus:ring-gray-500"
    );

    html! {
        <button
            data-collapse-toggle="mobile-menu"
            type="button"
            class={cl_button}
            onclick={props.onclick.clone()}
        >
            <svg class="w-6 h-6" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path></svg>
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavLinksProps {
    show: UseStateHandle<bool>,
    children: Children,
}

#[function_component(NavLinks)]
pub fn nav_links(props: &NavLinksProps) -> Html {
    let cl_ul = classes!(
        "flex",
        "flex-col",
        "mt-4",
        "rounded-lg",
        "md:flex-row",
        "md:space-x-4",
        "md:mt-0",
        "md:text-sm",
        "md:font-medium",
        "md:border-0",
        "md:bg-transparent",
        "bg-gray-800",
        "border-gray-700",
    );

    let cl_container = classes!("w-full", "md:block", "md:w-auto", "justify-start", "grow");

    html! {
        <div
            class={
                match *props.show {
                    true => cl_container,
                    false => classes!(cl_container, "hidden"),
                }
            }
            id="mobile-menu">
            <ul class={cl_ul}>
                { for props.children.iter() }
            </ul>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavLinkProps {
    label: String,
    route: Route,
}

#[function_component(NavLink)]
pub fn nav_link(props: &NavLinkProps) -> Html {
    let classes = classes!(
        "block",
        "py-2",
        "pr-4",
        "pl-3",
        "rounded",
        "md:border-0",
        "text-gray-400",
        "md:hover:text-white",
        "hover:bg-gray-700",
        "hover:text-white",
        "hover:rounded-md",
    );

    html! {
        <li>
          <Link<Route> to={props.route.clone()} {classes}>{props.label.clone()}</Link<Route>>
        </li>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavDropdownProps {
    label: String,
    children: Children,
}

#[function_component(NavDropdown)]
pub fn nav_dropdown(props: &NavDropdownProps) -> Html {
    let show = use_state(|| false);

    let onclick = {
        let show = show.clone();
        Callback::from(move |_: MouseEvent| show.set(!(*show)))
    };

    let cl_dropdown_button = classes!(
        "flex",
        "justify-between",
        "items-center",
        "py-2",
        "pr-4",
        "pl-3",
        "w-full",
        "font-medium",
        "rounded",
        "md:hover:bg-transparent",
        "md:border-0",
        "md:hover:text-blue-700",
        "md:p-0",
        "md:w-auto",
        "text-gray-400",
        "hover:text-white",
        "focus:text-white",
        "border-gray-700",
        "hover:bg-gray-700",
    );

    let cl_dropdown_container = classes!(
        "z-10",
        "w-44",
        "font-normal",
        "rounded",
        "divide-y",
        "shadow",
        "bg-gray-700",
        "divide-gray-600",
        "absolute",
        "top-11",
        "md:right-4"
    );

    html! {
    <div class="my-auto relative">
          <button
            id="dropdownNavbarLink"
            data-dropdown-toggle="dropdownNavbar"
            class={cl_dropdown_button}
            {onclick}
          >
          {props.label.clone()}
            <svg
              class="ml-1 w-5 h-5"
              fill="currentColor"
              viewBox="0 0 20 20"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                fill-rule="evenodd"
                d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                clip-rule="evenodd"
              ></path>
            </svg>
          </button>

          <div
            id="dropdownNavbar"
            class={
                match *show {
                    true => classes!(cl_dropdown_container, "block"),
                    false => classes!(cl_dropdown_container, "hidden"),
                }
            }
            // style="
            //   position: absolute;
            //   inset: 0px auto auto 0px;
            //   margin: 0px;
            //   transform: translate(427px, 66px);
            // "
            data-popper-reference-hidden=""
            data-popper-escaped=""
            data-popper-placement="bottom"
          >
            <ul
              class="py-1 text-sm text-gray-700 dark:text-gray-400"
              aria-labelledby="dropdownLargeButton"
            >
                { for props.children.iter() }
            </ul>
          </div>
        </div>
    }
}

#[function_component(NavDropdownLink)]
pub fn nav_dropdown_link(props: &NavLinkProps) -> Html {
    let classes = classes!(
        "block",
        "py-2",
        "px-4",
        "hover:bg-gray-600",
        "hover:text-white"
    );

    html! {
      <li>
            <Link<Route> to={props.route.clone()} {classes}>{props.label.clone()}</Link<Route>>
      </li>
    }
}

#[derive(Properties, PartialEq)]
pub struct UserMenuProps {
    pub user: SessionInfo,
}

#[function_component(UserMenu)]
pub fn user_menu(props: &UserMenuProps) -> Html {
    let display_name = use_state(String::new);

    {
        let username = props.user.username.clone();
        let display_name = display_name.clone();
        if props.user.is_auth {
            log::info!("Fetching user display name");
            use_effect_with_deps(
                move |_| {
                    if let Some(username) = username {
                        let url = format!("/api/users/{}", username);
                        wasm_bindgen_futures::spawn_local(async move {
                            match get::<User>(url).await {
                                Ok(u) => {
                                    display_name.set(u.display_name.unwrap_or(username));
                                }
                                Err(_) => (),
                            }
                        });
                    }
                    || ()
                },
                (),
            );
        }
    }

    if props.user.is_auth {
        html! {
            <NavDropdown label={(*display_name).clone()}>
                <NavDropdownLink label="Set Password" route={Route::SetPassword { username: props.user.username.clone().unwrap_or_default()} } />
                <NavDropdownLink label="Logout" route={Route::Logout} />
            </NavDropdown>
        }
    } else {
        html! {
            <NavLink label="Login" route={Route::Login}/>
        }
    }
}
