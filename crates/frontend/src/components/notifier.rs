use gloo_timers::callback::Timeout;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::types::Notification;

static ALERT: &'static str = "p-4 text-sm rounded-lg fade-in";
static INFO_ALERT: &'static str = "bg-blue-200 text-blue-800";
static WARN_ALERT: &'static str = "bg-yellow-200 text-yellow-800";
static ERR_ALERT: &'static str = "bg-red-200 text-red-800";

#[function_component(Notifier)]
pub fn notifier() -> Html {
    let (state, dispatch) = use_store::<Notification>();
    let msg = state.msg.clone();
    let lvl = state.lvl.clone();
    let dismiss = dispatch.reduce_mut_callback(|s| {
        s.msg = None;
        s.lvl = None;
    });

    {
        use_effect(move || {
            let timeout = Timeout::new(10000, move || {
                dispatch.reduce_mut(|s| {
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
            let mut classes = classes!(ALERT);
            match lvl {
                Some(l) => match l.as_str() {
                    "warn" => classes.push(WARN_ALERT),
                    "error" => classes.push(ERR_ALERT),
                    _ => classes.push(INFO_ALERT),
                },
                None => classes.push(INFO_ALERT),
            };

            html! {
                <div class="fixed bottom-2 left-2 ">
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
pub fn notify(msg: &str, lvl: String) {
    let dispatch = Dispatch::<Notification>::new();
    dispatch.reduce_mut(|s| {
        s.msg = Some(msg.into());
        s.lvl = Some(lvl);
    });
}

/// Sends an info notification to the user.
pub fn notify_info(msg: &str) {
    notify(msg, "info".to_string());
}
///
/// Sends a warning notification to the user.
pub fn notify_warn(msg: &str) {
    notify(msg, "warn".to_string());
}

/// Sends an error notification to the user.
pub fn notify_error(msg: &str) {
    notify(msg, "error".to_string());
}
