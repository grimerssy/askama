pub use axum_core::response::{IntoResponse, Response};
use http::{header::CONTENT_TYPE, HeaderValue, StatusCode};

use crate::Template;

pub fn into_response<T: Template>(t: &T) -> Response {
    match t.render() {
        Ok(body) => {
            let headers = [(CONTENT_TYPE, HeaderValue::from_static(T::MIME_TYPE))];
            (headers, body).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
