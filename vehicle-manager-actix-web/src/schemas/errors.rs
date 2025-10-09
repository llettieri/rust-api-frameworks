use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[display("E_INVALID_OBJECT_ID")]
    InvalidObjectId,
    #[display("E_NOT_FOUND")]
    NotFound,
}

impl ApiError {
    fn content_type(&self) -> ContentType {
        match &self {
            _ => ContentType::plaintext(),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match &self {
            ApiError::InvalidObjectId => StatusCode::BAD_REQUEST,
            ApiError::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type(self.content_type())
            .body(self.to_string())
    }
}
