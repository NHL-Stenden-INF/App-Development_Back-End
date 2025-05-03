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

pub struct Credentials
{
    email: String,
    password: String
}

pub async fn authenticate(
    headers: HeaderMap,
    request: Request,
    next: Next
) -> Result<Response, StatusCode>
{
    match headers.get("Authorization")
    {
        Some(header) => 
            {
                let credentials: Credentials = get_credentials(header);
                match validate_user(&credentials.email, &credentials.password) 
                { 
                    true => Ok(next.run(request).await),
                    false => Err(StatusCode::UNAUTHORIZED)
                }
            },
        None => Err(StatusCode::UNAUTHORIZED),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

pub fn get_credentials(authorization_header: &HeaderValue) -> Credentials
{
    let parameters = AUTH_PATTERN.captures(
        authorization_header
            .to_str()
            .unwrap_or_else(|_| { "Basic invalid:user" }))
        .unwrap();

    Credentials
    {
        email: parameters
            .get(1)
            .map_or("".to_string(), |m| m.as_str().to_string()),
        password: parameters
            .get(2)
            .map_or("".to_string(), |m| m.as_str().to_string())
    }
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