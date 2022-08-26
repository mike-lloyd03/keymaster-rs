use crate::routes::Route;
use crate::services::auth::get_session_info;
use crate::types::UserInfo;
use yew::prelude::*;
use yew_router::prelude::Link;
use yewdux::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    let (user_state, _) = use_store::<UserInfo>();
    let username = user_state.username.clone().unwrap_or_default();

    // Get user session info on load
    use_effect_with_deps(
        move |_| {
            get_session_info();
            || ()
        },
        (),
    );

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
                            if user_state.is_auth {
                                auth_links
                            } else {
                                html!{}
                            }
                        }
                    </ul>
                        {
                            if user_state.is_auth {
                                html!{
                                    <>
                                        <span class="navbar-text">
                                            {username}
                                        </span>
                                        <div class="nav-item">
                                            <Link<Route> to={Route::Logout}
                                                classes={classes!("nav-link")}
                                            >
                                                { "Logout" }
                                            </Link<Route>>
                                        </div>
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
