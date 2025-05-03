use axum::extract::Request;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use regex::Regex;
use lazy_static;
use rusqlite::named_params;
use sql_query_builder as sql;

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

    validate_user(email, password)
}

pub fn validate_user(email: &str, password: &str) -> bool
{
    let conn = crate::database::CONN.lock().unwrap();

    let query: String = sql::Select::new()
        .select("*")
        .from("users")
        .where_clause("email = :email")
        .as_string();

    let stmt = conn.prepare(&*query);
    let encrypted_password: String = stmt
        .unwrap()
        .query_row(named_params! {":email": email}, |row|
            {
                row.get::<usize, String> (3)
            })
        .unwrap_or_else(|error| {error.to_string()});
    
    match bcrypt::verify(password, &*encrypted_password)
    {
        Ok(_) => true,
        Err(_) => false
    }
}