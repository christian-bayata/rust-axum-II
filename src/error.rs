use axum::{
    Json, http::{StatusCode}, response::{IntoResponse, Response}
};
use serde::{Deserialize, Serialize};
use std::fmt::{self, write};

#[derive(Debug, Serialize, Deserialize)]
pub struct  ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    HashingError,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExist,
    UserNoLongerExist,
    TokenNotProvided,
    PermissionDenied,
    UserNotAuthenticated,
    InvalidHashFormat
}


impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorMessage::EmptyPassword =>
                write!(f, "Password cannot be empty"),
            ErrorMessage::ExceededMaxPasswordLength(len) =>
                write!(f, "Password exceeded max length of {}", len),
            ErrorMessage::HashingError =>
                write!(f, "Error hashing password"),
            ErrorMessage::InvalidToken =>
                write!(f, "Invalid token"),
            ErrorMessage::ServerError =>
                write!(f, "Internal server error"),
            ErrorMessage::WrongCredentials =>
                write!(f, "Wrong credentials"),
            ErrorMessage::EmailExist =>
                write!(f, "Email already exists"),
            ErrorMessage::UserNoLongerExist =>
                write!(f, "User no longer exists"),
            ErrorMessage::TokenNotProvided =>
                write!(f, "Token not provided"),
            ErrorMessage::PermissionDenied =>
                write!(f, "Permission denied"),
            ErrorMessage::UserNotAuthenticated =>
                write!(f, "User not authenticated"),
            ErrorMessage::InvalidHashFormat =>
                write!(f, "Invalid password hash format"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: StatusCode
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
        HttpError {
            message: message.into(),
            status
        }
    } 

    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError { 
            message: message.into(), 
            status: StatusCode::INTERNAL_SERVER_ERROR 
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError { 
            message: message.into(), 
            status: StatusCode::BAD_REQUEST 
        }
    }

    pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
        HttpError { 
            message: message.into(), 
            status: StatusCode::CONFLICT 
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        HttpError { 
            message: message.into(), 
            status: StatusCode::UNAUTHORIZED 
        }
    }

    pub fn into_http_response(self) -> Response {
        let json_response = Json(ErrorResponse {
            status: "fail".to_string(),
            message: self.message.clone(),
        });

        (self.status, json_response).into_response()
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpError: message: {}, status: {}", self.message, self.status)
    }
}