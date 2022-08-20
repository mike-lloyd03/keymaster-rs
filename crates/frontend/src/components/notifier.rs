use gloo_timers::callback::Timeout;
use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::{use_store, StoreRef};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Notification {
    pub msg: Option<String>,
    pub lvl: Option<String>,
}

#[function_component(Notifier)]
pub fn notifier() -> Html {
    let store = use_store::<BasicStore<Notification>>();
    let msg = store.state().map(|s| s.msg.clone()).unwrap_or_default();
    let lvl = store.state().map(|s| s.lvl.clone()).unwrap_or_default();
    let dismiss = store.dispatch().reduce_callback(|s| {
        s.msg = None;
        s.lvl = None;
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
        Some(message) if !message.is_empty() => {
            let mut classes = classes!("alert", "alert-float");
            match lvl {
                Some(l) => match l.as_str() {
                    "warn" => classes.push("alert-warning"),
                    "error" => classes.push("alert-danger"),
                    _ => classes.push("alert-info"),
                },
                None => classes.push("alert-info"),
            };

            html! {
                <div class="container fade-in">
                    <div
                        class={classes}
                        onclick={dismiss}
                        role="alert">
                        {message}
                    </div>
                </div>
            }
        }
        _ => html! {},
    }
}

/// Sends a notification to the user.
pub fn notify(store: StoreRef<BasicStore<Notification>>, msg: String, lvl: String) {
    store.dispatch().reduce(|s| {
        s.msg = Some(msg);
        s.lvl = Some(lvl);
    });
}
