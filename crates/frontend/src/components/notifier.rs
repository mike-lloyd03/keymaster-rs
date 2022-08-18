use yew::prelude::*;
use yewdux::prelude::*;
use yewdux::store::Store;

#[derive(Properties, Clone, Default, PartialEq, Eq, Store)]
pub struct Notification {
    pub msg: Option<String>,
    pub lvl: Option<String>,
}

#[function_component(Notifier)]
pub fn notifier() -> Html {
    let (state, dispatch) = use_store::<Notification>();
    let dismiss = dispatch.set_callback(|_| Notification {
        ..Default::default()
    });

    html! {
        <div class="container">
            <div
                class={match state.lvl.as_str() {
                    "warn" => "alert alert-warning",
                    "error" =>  "alert alert-danger",
                    _ => "alert alert-info",
                }}
                onclick={dismiss}
                role="alert">
                {state.msg}
            </div>
        </div>
    }
}
