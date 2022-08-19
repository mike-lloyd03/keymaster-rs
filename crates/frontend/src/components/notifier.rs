use gloo_timers::callback::Timeout;
use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::use_store;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Notification {
    pub msg: Option<String>,
    pub lvl: Option<String>,
}

#[function_component(Notifier)]
pub fn notifier() -> Html {
    let store = use_store::<BasicStore<Notification>>();
    let msg = store.state().map(|s| s.msg.clone()).unwrap_or(None);
    let lvl = store.state().map(|s| s.lvl.clone()).unwrap_or(None);
    let dismiss = store.dispatch().reduce_callback(|_| Notification {
        ..Default::default()
    });

    {
        use_effect(move || {
            let timeout = Timeout::new(10000, move || {
                store.dispatch().reduce(|s| {
                    s.msg = None;
                    s.lvl = None;
                });
            });
            || {
                timeout.cancel();
            }
        });
    }

    match msg {
        Some(message) => html! {
            <div class="container fade-in">
                <div
                    class={match lvl.unwrap_or_else(||"".to_string()).as_str() {
                        "warn" => "alert alert-float alert-warning",
                        "error" =>  "alert alert-float alert-danger",
                        _ => "alert alert-float alert-info",
                    }}
                    onclick={dismiss}
                    role="alert">
                    {message}
                </div>
            </div>
        },
        None => html! {},
    }
}
