use axum::Json;
use axum::extract::Request;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use rusqlite::named_params;
use sql_query_builder as sql;
use crate::user::User;
use base64::{Engine as _, alphabet, engine::{self, general_purpose}, DecodeError};

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
                let credentials: Credentials = match get_credentials(header)
                {
                    Some(credentials) => credentials, 
                    None => {return Err(StatusCode::BAD_REQUEST)}
                };
                match validate_user(&credentials.email, &credentials.password)
                {
                    true => Ok(next.run(request).await),
                    false => Err(StatusCode::UNAUTHORIZED)
                }
            },
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

pub fn get_credentials(authorization_header: &HeaderValue) -> Option<Credentials>
{
    let auth_header_string: String = match authorization_header.to_str()
    {
        Ok(string_reference) => string_reference.to_string(),
        Err(_) => "Basic aW52YWxpZDp1c2VyCg==".to_string()
    };

    if !auth_header_string.starts_with("Basic ")
    {
        return None;
    }

    let auth_string: String = match general_purpose::STANDARD.decode(&auth_header_string[6..])
    {
        Ok(vector_string) => String::from_utf8(vector_string).unwrap_or_else(|_| "invalid:user".to_string()),
        Err(_) => "invalid:user".to_string()
    };
    
    let mut credential = auth_string.split(":");

    Some(Credentials
    {
        email: credential.next().unwrap().to_string(),
        password: credential.next().unwrap().to_string(),
    })
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

pub fn get_user_from_header(header_map: HeaderMap) -> Result<User, String>
{
    let authorization_header = match header_map.get("Authorization")
    {
        Some(header) => header,
        None => {return Err("Unable to find user with these credentials".to_string())},
    };

    let credentials: Credentials = match get_credentials(authorization_header)
    {
        Some(credentials) => credentials,
        None => {return Err("Unable to decode header information".to_string())}
    };

    let query: String = sql::Select::new()
        .select("*")
        .from("users")
        .where_clause("email = :email")
        .as_string();

    let conn = crate::database::CONN.lock().unwrap();
    let stmt = conn.prepare(&*query);
    
    let mut password_hash: String = String::new();
    let fetched_user: User = match stmt.unwrap()
        .query_row(named_params! {":email": credentials.email}, |row|
            {
                password_hash = row.get::<usize, String>(3).unwrap();
                Ok(User::new(
                    row.get::<usize, i32>(0).unwrap(),
                    row.get::<usize, String>(1).unwrap(),
                    row.get::<usize, String>(2).unwrap(),
                    row.get::<usize, i32>(4).unwrap())
                )
            })
    {
        Ok(user) => user,
        Err(error) => {return Err(format!("Unable to authenticate user: {}", error))}
    };
    
    if verify(&*credentials.password, &*password_hash)
    {
        let result = fetched_user;
        return Ok(result)
    }
    
    Err("Unable to authenticate user: Incorrect password".to_string())
}

pub async  fn get_user_from_header_json(headers: HeaderMap) -> Result<Json<User>, (StatusCode, String)>
{
    match get_user_from_header(headers) {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Couldn't authenticate user".to_string()))
    }
}

fn verify(password: &str, hashed_password: &str) -> bool
{
    match bcrypt::verify(password, hashed_password)
    {
        Ok(_) => true,
        Err(_) => false,
    }
}