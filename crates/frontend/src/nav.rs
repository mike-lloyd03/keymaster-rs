use yew::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
    <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
        <div class="container-fluid">
            <a class="navbar-brand text-primary" href="/">{ "KeyMaster" }</a>
            <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                <span class="navbar-toggler-icon"></span>
            </button>
            <div class="collapse navbar-collapse" id="navbarSupportedContent">
                <ul class="navbar-nav me-auto mb-2 mb-lg-0">

                    <li class="nav-item">
                      <a class="nav-link" href="/">{ "Home" }</a>
                    </li>

                    <li class="nav-item">
                      <a class="nav-link" href="/assign-key">{ "Assign Key" }</a>
                    </li>

                    <li class="nav-item dropdown">
                      <a class="nav-link dropdown-toggle" href="#" id="navbarDropdownMenuLink" role="button" data-toggle="dropdown" aria-expanded="false">
                          { "Configuration" }
                      </a>
                      <ul class="dropdown-menu dropdown-menu-dark" aria-labelledby="navbarDropdownMenuLink">
                        <li><a class="dropdown-item" href="/assignments">{ "Assignments" }</a></li>
                        <li><a class="dropdown-item" href="/keys">{ "Keys" }</a></li>
                        <li><a class="dropdown-item" href="/users">{ "Users" }</a></li>
                      </ul>
                    </li>

                    <li class="nav-item">
                      <a class="nav-link" href="/logout">{ "Logout" }</a>
                    </li>

                </ul>
            </div>
        </div>
    </nav>
    }
}
