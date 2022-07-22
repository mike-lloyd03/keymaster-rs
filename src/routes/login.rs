use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <div class="container text-light my-3" style="max-width: 600px;">
            <div class="row justify-content-center">
                <div>
                    <h1>{ "Login" }</h1>
                    <form action="" method="post" class="form" role="form">
                        <input id="csrf_token" name="csrf_token" type="hidden" value="ImEzOThlODZhMzZkYzllZWI0ZTQ0OGU1ZDQ1N2UxYTE0YTA0NWE5N2Ii.Yto3Cg.ZKSkYoghUGGnZkkuO0p4vScIPFs" />
                        <div class="form-group  required"><label class="control-label" for="username">{ "Username" }</label>
                            <input class="form-control" id="username" name="username" required=true type="text" value="" />
                        </div>
                        <div class="form-group  required"><label class="control-label" for="password">{ "Password" }</label>
                            <input class="form-control" id="password" name="password" required=true type="password" value="" />
                        </div>
                        <input class="btn btn-primary" id="submit" name="submit" type="submit" value="Sign In" />
                    </form>
                </div>
            </div>
        </div>
    }
}
