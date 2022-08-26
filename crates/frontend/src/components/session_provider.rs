use crate::services::requests::get;
use crate::types::SessionInfo;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SessionProviderProps {
    pub children: Children,
}

#[function_component(SessionProvider)]
pub fn session_provider(props: &SessionProviderProps) -> Html {
    let (state, dispatch) = use_store::<SessionInfo>();

    log::info!("Mounting SessionProvider");

    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                log::info!("Getting session info: {:?}", &dispatch.get());
                let ui: SessionInfo = get("/api/session".into()).await.unwrap();
                dispatch.reduce_mut(|s| {
                    s.username = ui.username;
                    s.is_auth = ui.is_auth;
                    s.is_admin = ui.is_admin;
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
