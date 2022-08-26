use crate::{
    components::notifier::notify_error,
    error::Error,
    routes::Route,
    types::{Credentials, SessionInfo},
};
use yew_router::prelude::*;
use yewdux::prelude::*;

use super::requests::{get, post};

// pub fn get_session_info() {
//     let dispatch = Dispatch::<SessionInfo>::new();
//     wasm_bindgen_futures::spawn_local(async move {
//         let ui: SessionInfo = get("/api/session".into()).await.unwrap();
//         dispatch.reduce_mut(|s| {
//             s.username = ui.username;
//             s.is_auth = ui.is_auth;
//             s.is_admin = ui.is_admin;
//         });
//     });
// }

pub fn set_session_info(ui: SessionInfo) {
    let dispatch = Dispatch::<SessionInfo>::new();
    dispatch.reduce_mut(|s| {
        s.username = ui.username;
        s.is_auth = ui.is_auth;
        s.is_admin = ui.is_admin;
        s.loading = false;
    });
}

pub async fn clear_session_info() {
    let dispatch = Dispatch::<SessionInfo>::new();
    dispatch.reduce_mut(|s| {
        s.username = None;
        s.is_auth = false;
        s.is_admin = false;
        s.loading = false;
    });
}

pub async fn login_user(creds: Credentials, history: &AnyHistory) {
    match post::<Credentials, SessionInfo>("/api/login".into(), creds).await {
        Ok(ui) => {
            set_session_info(ui);
            history.push(Route::Home)
        }
        Err(e) => {
            if e == Error::Unauthorized {
                notify_error("Invalid credentials");
            } else {
                notify_error(&e.to_string());
            }
        }
    }
}

pub async fn logout_user() {
    if let Err(e) = post::<(), String>("/api/logout".into(), ()).await {
        log::error!("{}", e);
    }
    clear_session_info().await;
}

pub fn current_user() -> SessionInfo {
    let dispatch = Dispatch::<SessionInfo>::new();

    (*dispatch.get()).clone()
}
