use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use error_stack::Report;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Bad Request")]
    BadRequest,
    #[error("Internal Server Error")]
    InternalServerError,
}

// `ApiErrorResponse` is a newtype wrapper around `error_stack::Report<ApiError>`.
// This is necessary to work around Rust's "orphan rule". The rule prevents us from
// implementing a foreign trait (like `axum::response::IntoResponse`) on a foreign type
// (`error_stack::Report`).
//
// By wrapping `Report` in our own local `ApiErrorResponse` struct, we gain the ability
// to implement `IntoResponse` for it, allowing us to define a centralized conversion
// from our application's error reports into HTTP responses.
pub struct ApiErrorResponse(pub Report<ApiError>);

impl IntoResponse for ApiErrorResponse {
    fn into_response(self) -> Response {
        let report = self.0;

        // Print the full error report to the console for debugging.
        println!("{report:?}");

        let status = match report.current_context() {
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, report.to_string()).into_response()
    }
}

// This implementation allows Axum's `Json` extractor to automatically convert
// a JSON parsing failure (`JsonRejection`) into our `ApiErrorResponse` type.
impl From<JsonRejection> for ApiErrorResponse {
    fn from(rejection: JsonRejection) -> Self {
        let report = Report::new(ApiError::BadRequest)
            .attach(format!("Failed to parse JSON body: {rejection}"));
        Self(report)
    }
}

// This implementation provides an easy way to convert a raw `Report<ApiError>`
// into our `ApiErrorResponse` wrapper, typically by calling `.into()`.
impl From<Report<ApiError>> for ApiErrorResponse {
    fn from(report: Report<ApiError>) -> Self {
        Self(report)
    }
}
