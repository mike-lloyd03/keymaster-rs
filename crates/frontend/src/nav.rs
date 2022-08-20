use crate::routes::Route;
use crate::types::UserInfo;
use yew::prelude::*;
use yew_router::prelude::Link;
use yewdux::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    let (user_state, _) = use_store::<UserInfo>();
    let username = user_state.username.clone().unwrap_or_default();

    html! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
            <div class="container-fluid">
                <Link<Route> to={Route::Home} classes={classes!("navbar-brand", "text-primary")}>{ "KeyMaster" }</Link<Route>>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <ul class="navbar-nav me-auto mb-2 mb-lg-0">

                        <li class="nav-item">
                            <Link<Route> to={Route::Home} classes={classes!("nav-link")}>{ "Home" }</Link<Route>>
                        </li>

                        <li class="nav-item">
                            <Link<Route> to={Route::AssignKey} classes={classes!("nav-link")}>{ "Assign Key" }</Link<Route>>
                        </li>

                        <li class="nav-item dropdown">
                          <a class="nav-link dropdown-toggle" href="#" id="navbarDropdownMenuLink" role="button" data-toggle="dropdown" aria-expanded="false">
                              { "Configuration" }
                          </a>
                          <ul class="dropdown-menu dropdown-menu-dark" aria-labelledby="navbarDropdownMenuLink">
                            <li><Link<Route> to={Route::Assignments} classes={classes!("dropdown-item")}>{ "Assignments" }</Link<Route>></li>
                            <li><Link<Route> to={Route::Keys} classes={classes!("dropdown-item")}>{ "Keys" }</Link<Route>></li>
                            <li><Link<Route> to={Route::Users} classes={classes!("dropdown-item")}>{ "Users" }</Link<Route>></li>
                          </ul>
                        </li>

                        <li class="nav-item">
                            <Link<Route> to={Route::Login} classes={classes!("nav-link")}>{ "Login" }</Link<Route>>
                        </li>

                        <li class="nav-item">
                            <Link<Route> to={Route::Logout} classes={classes!("nav-link")}>{ "Logout" }</Link<Route>>
                        </li>

                    </ul>
                    <span class="navbar-text">
                        {username}
                    </span>
                </div>
            </div>
        </nav>
    }
}
