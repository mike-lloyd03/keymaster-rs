use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Notification {
    pub message: String,
    pub level: String,
}

impl Default for Notification {
    fn default() -> Self {
        Self {
            message: "".to_string(),
            level: "info".to_string(),
        }
    }
}

#[function_component(Notifier)]
pub fn notifier(props: &Notification) -> Html {
    html! {
        <div class="container">
            <div
                class={match props.level.clone().as_str() {
                    "warn" => "alert alert-warning",
                    "error" =>  "alert alert-danger",
                    _ => "alert alert-info",
                }}
                role="alert">
                {props.message.clone()}
            </div>
        </div>
    }
}
