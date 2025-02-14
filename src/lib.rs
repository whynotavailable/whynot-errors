use std::fmt::Display;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
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
    pub fn new(code: StatusCode, message: impl ToString) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }

    pub fn not_found() -> Self {
        Self {
            code: StatusCode::NOT_FOUND,
            message: "Not Found".to_string(),
        }
    }

    pub fn server_error(message: impl ToString) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.to_string(),
        }
    }

    pub fn bad_request(message: impl ToString) -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: message.to_string(),
        }
    }

    /// implementing this here instead of a trait fixes conflict issues
    pub fn from(obj: impl ToString) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: obj.to_string(),
        }
    }

    /// Return a closure which will accept a ToString to generate an AppError
    pub fn fact<T: ToString>(code: StatusCode) -> impl Fn(T) -> Self {
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

/// If you are returning JSON, use this as well.
pub type JsonResult<T> = AppResult<Json<T>>;

/// Shortcut to wrap a result in json. Will consume the input, but that shouldn't matter.
pub fn json_ok<T>(obj: T) -> JsonResult<T> {
    Ok(Json(obj))
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
        let err2: AppError = AppError::from("hi");

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
        assert_eq!(AppError::new(StatusCode::FORBIDDEN, "hi").message, "hi");
        assert_eq!(
            AppError::new(StatusCode::FORBIDDEN, "hi".to_string()).message,
            "hi"
        );
    }

    #[test]
    fn test_fact() {
        let r: Result<(), String> = Err("hi".to_string());
        let mapped = r.map_err(AppError::fact(StatusCode::METHOD_NOT_ALLOWED));

        assert!(mapped.is_err());

        let e = mapped.unwrap_err();

        assert_eq!(e.code, StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(e.message, "hi");
    }
}
