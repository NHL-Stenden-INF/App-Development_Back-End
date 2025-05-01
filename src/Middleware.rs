use crate::Database;

use axum::extract::Request;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use regex::{Matches, Regex};
use lazy_static;

lazy_static::lazy_static!
{
    static ref AUTH_PATTERN: Regex = Regex::new(r"Basic (.*):(.*)").unwrap();
}

pub async fn authenticate(
    headers: HeaderMap,
    request: Request,
    next: Next
) -> Result<Response, StatusCode>
{
    match headers.get("Authorization")
    {
        Some(header) if verify_credentials(header) => {Ok(next.run(request).await)},
        None => Err(StatusCode::UNAUTHORIZED),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

fn verify_credentials(authorization_header: &HeaderValue) -> bool
{
    let parameters = AUTH_PATTERN.captures(
        authorization_header
            .to_str()
            .unwrap_or_else(|_| {"Basic invalid:user"}))
        .unwrap();

    let email: &str = parameters.get(1).map_or("", |m| m.as_str());
    let password: &str = parameters.get(2).map_or("", |m| m.as_str());

    verify_credentials_by_string(email, password)
}

fn verify_credentials_by_string(email: &str, password: &str) -> bool
{
    Database::validate_user(email, password)
}