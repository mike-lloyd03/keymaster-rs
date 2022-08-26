use crate::{
    components::notifier::notify_error,
    error::Error,
    routes::Route,
    types::{Credentials, UserInfo},
};
use yew_router::prelude::*;
use yewdux::prelude::*;

use super::requests::{get, post};

pub async fn get_session_info() {
    let dispatch = Dispatch::<UserInfo>::new();
    let ui: UserInfo = get("/api/session".into()).await.unwrap();
    dispatch.reduce_mut(|s| {
        s.username = ui.username;
        s.is_auth = ui.is_auth;
        s.is_admin = ui.is_admin;
    });
}

pub async fn clear_session_info(dispatch: &Dispatch<UserInfo>) {
    dispatch.reduce_mut(|s| {
        s.username = None;
        s.is_auth = false;
        s.is_admin = false;
    });
}

pub async fn login_user(creds: Credentials, history: &AnyHistory) -> Result<(), Error> {
    match post::<Credentials, String>("/api/login".into(), creds).await {
        Ok(_) => {
            get_session_info().await;
            history.push(Route::Home)
        }
        Err(e) => {
            if e == Error::Unauthorized {
                notify_error("Invalid credentials");
            } else {
                notify_error(&e.to_string());
            }
            return Err(e);
        }
    }
    Ok(())
}

pub async fn logout_user() {
    let dispatch = Dispatch::<UserInfo>::new();
    if let Err(e) = post::<(), String>("/api/logout".into(), ()).await {
        log::error!("{}", e);
    }
    clear_session_info(&dispatch).await;
}
