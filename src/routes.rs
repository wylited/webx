use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{errors::ApiError, templates};

pub async fn home() -> impl IntoResponse {
    templates::IndexTemplate
}

pub async fn styles() -> Result<impl IntoResponse, ApiError> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../templates/output.css").to_owned())?;

    Ok(response)
}
