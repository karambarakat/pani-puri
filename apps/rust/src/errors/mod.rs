use actix_web::{
    body::BoxBody,
    dev::Response,
    http::{header::ContentType, StatusCode},
    Handler, HttpRequest, HttpResponse, HttpResponseBuilder, Responder, ResponseError,
};

use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, derive_more::Display, Error)]
pub enum MyErrors {
    #[display(fmt = "email already exists: {0}", _0)]
    EmailAlreadyExists(String),
    #[display(fmt = "email or password is incorrect")]
    EmailOrPasswordIncorrect,
    #[display(fmt = "validation error: {0}", _0)]
    ValidationError(String),
    /// ultimately, we should not use this error, improve the code or provide more a descriptive error
    #[display(fmt = "internal error")]
    UnknownError,
    #[display(fmt = "404 not found")]
    NotFound,
}

use convert_case::{Case, Casing};

impl Responder for MyErrors {
    type Body = BoxBody;
    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        self.error_response()
    }
}

impl ResponseError for MyErrors {
    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();

        let body = serde_json::json!({
            "data": null,
            "error": {
                "status": status.as_u16(),
                "code": status.canonical_reason().unwrap_or("UnknownError").to_case(Case::Pascal),
                "message": self.to_string(),
                "info": null
            }
        });

        HttpResponse::build(status)
            .insert_header(ContentType::json())
            .body(body.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            MyErrors::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
            MyErrors::ValidationError(_) => StatusCode::BAD_REQUEST,
            MyErrors::EmailOrPasswordIncorrect => StatusCode::UNAUTHORIZED,
            MyErrors::EmailAlreadyExists(_) => StatusCode::CONFLICT,
            MyErrors::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

pub mod basic_token_error {
    use actix_web::{
        http::{header::ContentType, StatusCode},
        HttpResponse, ResponseError,
    };

    use std::fmt::Debug;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, thiserror::Error)]
    pub struct BasicTokenRequired(pub String);

    impl std::fmt::Display for BasicTokenRequired {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "internal error, please try again")
        }
    }

    impl ResponseError for BasicTokenRequired {
        fn error_response(&self) -> HttpResponse {
            let status = StatusCode::BAD_REQUEST;

            let code = "basic_token_required";

            let body = serde_json::json!({
                "data": null,
                "error": {
                    "status": status.as_u16(),
                    "code": code,
                    "message": self.to_string(),
                    "info": {
                        "reason": self.0
                    }
                }
            });

            HttpResponse::build(status)
                .insert_header(ContentType::json())
                .body(body.to_string())
        }
    }
}

pub mod bearer_token_error {
    use actix_web::{
        http::{header::ContentType, StatusCode},
        HttpResponse, ResponseError,
    };

    use std::fmt::Debug;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, thiserror::Error, derive_more::Display)]
    pub enum BearerTokenErr {
        #[display(fmt = "token expired")]
        Expired,
        #[display(fmt = "user not found")]
        UserNotFound,
        #[display(fmt = "internal error, please try again")]
        ForDev(String),
    }

    impl ResponseError for BearerTokenErr {
        fn error_response(&self) -> HttpResponse {
            let status = StatusCode::BAD_REQUEST;

            let code = "bearer_token_error";

            let info = match self {
                BearerTokenErr::ForDev(reason) => {
                    serde_json::json!({
                        "reason": reason
                    })
                }
                _ => serde_json::json!(null),
            };

            let body = serde_json::json!({
                "data": null,
                "error": {
                    "status": status.as_u16(),
                    "code": code,
                    "message": self.to_string(),
                    "info": info
                }
            });

            HttpResponse::build(status)
                .insert_header(ContentType::json())
                .body(body.to_string())
        }
    }
}
