use crate::error::Error;
use gloo_net::http::{Method, Request};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub async fn request<T, U>(method: Method, url: String, body: T) -> Result<U, Error>
where
    T: Serialize + 'static,
    U: DeserializeOwned,
{
    let resp = match method {
        Method::GET | Method::DELETE => Request::new(&url).method(method).send().await,
        Method::POST => match Request::new(&url).method(method).json(&body) {
            Ok(req) => req.send().await,
            Err(_) => return Err(Error::SerializationError),
        },
        _ => unimplemented!(), // We're not using any other methods
    };

    match resp {
        Ok(data) => {
            if data.ok() {
                match data.json::<U>().await {
                    Ok(d) => Ok(d),
                    Err(_) => Err(Error::DeserializationError),
                }
            } else {
                let resp_text = data.text().await.unwrap_or_else(|_| "unknown error".into());
                Err(match data.status() {
                    400 => Error::BadRequest(resp_text),
                    401 => Error::Unauthorized,
                    404 => Error::NotFound(resp_text),
                    500 => Error::InternalServerError(resp_text),
                    _ => Error::RequestError(resp_text),
                })
            }
        }
        Err(e) => {
            let error_text = format!("{:?}", e);
            Err(Error::InternalServerError(error_text))
        }
    }
}

pub async fn get<U>(url: String) -> Result<U, Error>
where
    U: DeserializeOwned,
{
    request(Method::GET, url, ()).await
}

pub async fn post<T, U>(url: String, body: T) -> Result<U, Error>
where
    T: Serialize + 'static,
    U: DeserializeOwned,
{
    request(Method::POST, url, body).await
}

pub async fn delete<U>(url: String) -> Result<U, Error>
where
    U: DeserializeOwned,
{
    request(Method::DELETE, url, ()).await
}
