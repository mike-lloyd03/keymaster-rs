use crate::services::requests::get;
use crate::types::SessionInfo;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SessionProviderProps {
    pub children: Children,
}

#[function_component(SessionProvider)]
pub fn session_provider(props: &SessionProviderProps) -> Html {
    let (_, dispatch) = use_store::<SessionInfo>();

    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                log::info!("Getting session info: {:?}", &dispatch.get());
                let ui: SessionInfo = get("/api/session".into()).await.unwrap();
                dispatch.reduce_mut(|s| {
                    s.username = ui.username;
                    s.is_auth = ui.is_auth;
                    s.is_admin = ui.is_admin;
                    s.fetched = true;
                });
            });
            || ()
        },
        (),
    );

    html!(
        { for props.children.iter() }
    )
}

#[function_component(SessionMonitor)]
pub fn session_monitor() -> Html {
    let (state, _) = use_store::<SessionInfo>();

    let classes = classes!("position-static", "bg-white");

    html! {
        <div class={classes}>
            <p>{format!("username: {}", state.username.clone().unwrap_or_default())}</p>
            <p>{format!("is_auth: {}", state.is_auth.clone())}</p>
            <p>{format!("is_admin: {}", state.is_admin.clone())}</p>
            <p>{format!("fetched: {}", state.fetched.clone())}</p>
        </div>
    }
}
