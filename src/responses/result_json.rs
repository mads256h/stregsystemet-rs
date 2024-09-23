use std::error::Error;

use axum::{
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
};
use bytes::{BufMut, BytesMut};
use derive_more::derive::{From, Into};
use serde::Serialize;

#[derive(Debug, Clone, Copy, From, Into)]
#[must_use]
pub struct ResultJson<T, E>(pub Result<T, E>);

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(tag = "status", content = "content")]
enum ResponseWrapper<T, E>
where
    T: Serialize,
    E: Serialize,
    E: HttpStatusCode,
{
    Ok(T),
    Error(E),
}

impl<T, E> ResponseWrapper<T, E>
where
    T: Serialize,
    E: Serialize,
    E: HttpStatusCode,
{
    fn status_code(&self) -> StatusCode {
        match self {
            ResponseWrapper::Ok(_) => StatusCode::OK,
            ResponseWrapper::Error(err) => err.status_code(),
        }
    }
}

impl<T, E> IntoResponse for ResultJson<T, E>
where
    T: Serialize,
    E: Error,
    E: Serialize,
    E: HttpStatusCode,
{
    fn into_response(self) -> axum::response::Response {
        // Much code is copied from axum/json.rs
        let mut buf = BytesMut::with_capacity(128).writer();

        let response_wrapper = match self.0 {
            Ok(ok) => ResponseWrapper::Ok(ok),
            Err(err) => ResponseWrapper::Error(err),
        };

        match serde_json::to_writer(&mut buf, &response_wrapper) {
            Ok(()) => (
                response_wrapper.status_code(),
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                )],
                buf.into_inner().freeze(),
            )
                .into_response(),

            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                )],
                err.to_string(),
            )
                .into_response(),
        }
    }
}

pub trait HttpStatusCode {
    fn status_code(&self) -> StatusCode;
}
