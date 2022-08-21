use crate::{
    components::notifier::notify,
    error::Error,
    routes::Route,
    types::{Credentials, Notification, UserInfo},
};
use yew_router::prelude::*;
use yewdux::prelude::*;

use super::requests::{get, post};

pub async fn get_session_info(dispatch: &Dispatch<UserInfo>) {
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

pub async fn login_user(
    creds: Credentials,
    user_dispatch: &Dispatch<UserInfo>,
    notify_dispatch: &Dispatch<Notification>,
    history: &AnyHistory,
) -> Result<(), Error> {
    match post::<Credentials, String>("/api/login".into(), creds).await {
        Ok(_) => {
            get_session_info(user_dispatch).await;
            history.push(Route::Home)
        }
        Err(e) => {
            if e == Error::Unauthorized {
                notify(
                    notify_dispatch,
                    "Invalid credentials".into(),
                    "error".into(),
                );
            } else {
                notify(notify_dispatch, e.to_string(), "error".into());
            }
            return Err(e);
        }
    }
    Ok(())
}

pub async fn logout_user(dispatch: &Dispatch<UserInfo>) {
    if let Err(e) = post::<(), String>("/api/logout".into(), ()).await {
        log::error!("{}", e);
    }
    clear_session_info(dispatch).await;
}
