use gloo_timers::callback::Timeout;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::types::Notification;

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
pub fn notify(dispatch: &Dispatch<Notification>, msg: String, lvl: String) {
    dispatch.reduce_mut(|s| {
        s.msg = Some(msg);
        s.lvl = Some(lvl);
    });
}
