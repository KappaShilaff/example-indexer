use aide::OperationOutput;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use hex::FromHexError;


pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("an error occurred with the database")]
    Sqlx(#[from] sqlx::Error),

    #[error("an internal server error occurred: {0}")]
    Anyhow(anyhow::Error),

    #[error("hex error occurred")]
    Decode(#[from] FromHexError),

    #[error("serde error occurred")]
    Serde(#[from] serde_json::Error),

    #[error("not found")]
    NotFound,
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        let error_text = e.to_string();
        if error_text.contains("not found") || error_text.contains("cant find pool") || error_text.contains("none pair") {
            Self::NotFound
        } else {
            Self::Anyhow(e)
        }
    }
}


// cant find pool

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Sqlx(_) | Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Decode(_) | Self::Serde(_) => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
/// Axum allows you to return `Result` from handler functions, but the error type
/// also must be some sort of response type.
///
/// By default, the generated `Display` impl is used to return a plaintext error message
/// to the client.
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::Sqlx(ref e) => {
                log::error!("SQLx error: {:?}", e);
            }

            Self::Anyhow(ref e) => {
                if !e.to_string().contains("empty") {
                    log::error!("Generic error: {:?}", e);
                }
            }
            Self::Decode(ref e) => {
                log::error!("Failed to decode string {:?}", e);
            }
            Self::Serde(ref e) => {
                log::error!("Failed to deserialize string {:?}", e);
            }
            Error::NotFound => {}
        }

        (self.status_code(), Json(serde_json::json!({"reason":self.to_string()}))).into_response()
    }
}

impl OperationOutput for Error {
    type Inner = Self;
}
