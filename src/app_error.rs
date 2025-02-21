use std::fmt::Display;

use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::Json;

/// Global error type
/// Use in basically all scenarios where an error is needed.
#[derive(Debug)]
pub struct AppError {
    pub code: StatusCode,
    pub message: String,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Code: {}; {};", self.code.as_u16(), self.message)
    }
}

impl AppError {
    /// Create a new `AppError` from any `ToString` with a code 500.
    /// If you want to customize the code, use the `AppError::code` factory.
    pub fn new(obj: impl ToString) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: obj.to_string(),
        }
    }

    /// FIXME: Remove this prior to version 1
    #[deprecated]
    pub fn from(obj: impl ToString) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: obj.to_string(),
        }
    }

    /// Return a closure which will accept a ToString to generate an AppError
    pub fn code<T: ToString>(code: StatusCode) -> impl Fn(T) -> Self {
        move |obj| Self {
            code,
            message: obj.to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.code, self.message).into_response()
    }
}

/// Use this for most functions that return a result
pub type AppResult<T> = Result<T, AppError>;

/// If you are returning JSON, use this.
pub type JsonResult<T> = AppResult<Json<T>>;

/// Shortcut to wrap a result in json. Will consume the input.
pub fn json_ok<T>(obj: T) -> JsonResult<T> {
    Ok(Json(obj))
}

/// If you are returning HTML, use this.
pub type HtmlResult = AppResult<Html<String>>;

/// Shortcut to wrap a result in html. Will consume the input.
pub fn html_ok(s: impl ToString) -> HtmlResult {
    Ok(Html(s.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        let err = AppError {
            code: StatusCode::OK,
            message: "ok".to_string(),
        };

        assert_eq!(err.to_string(), "Code: 200; ok;");
    }

    /// Test the from method. It should make an error from any object that implements `Display`
    #[test]
    fn test_from() {
        let err2: AppError = AppError::new("hi");

        assert_eq!(err2.message, "hi");
        assert_eq!(err2.code, StatusCode::INTERNAL_SERVER_ERROR);
    }

    /// Test that the types are all correct for `json_ok`.
    #[test]
    fn test_json() {
        let resp: JsonResult<String> = json_ok("hi".to_string());
        assert_eq!(resp.unwrap().to_string(), "hi");
    }

    #[test]
    fn test_traits() {
        assert_eq!(AppError::new("hi").message, "hi");
        assert_eq!(AppError::new("hi".to_string()).message, "hi");
    }

    #[test]
    fn test_code() {
        let r: Result<(), String> = Err("hi".to_string());
        let mapped = r.map_err(AppError::code(StatusCode::METHOD_NOT_ALLOWED));

        assert!(mapped.is_err());

        let e = mapped.unwrap_err();

        assert_eq!(e.code, StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(e.message, "hi");
    }
}
