use axum::extract::Request;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use rusqlite::named_params;
use sql_query_builder as sql;
use crate::user::User;
use base64::{Engine as _, engine::general_purpose};

pub struct Credentials
{
    pub email: String,
    pub password: String
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
    }
}

pub fn get_credentials(authorization_header: &HeaderValue) -> Credentials
{
    let header_str = match authorization_header.to_str() {
        Ok(s) => s,
        Err(_) => return Credentials { email: "".to_string(), password: "".to_string() },
    };

    if !header_str.starts_with("Basic ") {
        return Credentials { email: "".to_string(), password: "".to_string() };
    }

    let b64_credentials = &header_str[6..];

    let decoded_bytes = match general_purpose::STANDARD.decode(b64_credentials) {
        Ok(bytes) => bytes,
        Err(_) => return Credentials { email: "".to_string(), password: "".to_string() },
    };

    let decoded_str = match String::from_utf8(decoded_bytes) {
        Ok(s) => s,
        Err(_) => return Credentials { email: "".to_string(), password: "".to_string() },
    };

    let parts: Vec<&str> = decoded_str.splitn(2, ':').collect();
    if parts.len() == 2 {
        Credentials {
            email: parts[0].to_string(),
            password: parts[1].to_string(),
        }
    } else {
        Credentials { email: "".to_string(), password: "".to_string() }
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

pub fn get_user_from_header(header_map: HeaderMap) -> Result<User, String>
{
    let authorization_header = match header_map.get("Authorization")
    {
        Some(header) => header,
        None => {return Err("Unable to find user with these credentials".to_string())},
    };

    let credentials: Credentials = get_credentials(authorization_header);

    let query: String = sql::Select::new()
        .select("*")
        .from("users")
        .where_clause("email = :email")
        .as_string();

    let conn = crate::database::CONN.lock().unwrap();
    let stmt = conn.prepare(&*query);
    
    let fetched_user: User = match stmt.unwrap()
        .query_row(named_params! {":email": credentials.email}, |row|
            {
                Ok(User::new(
                    row.get::<usize, i32>(0).unwrap(),
                    row.get::<usize, String>(1).unwrap(),
                    row.get::<usize, String>(2).unwrap(),
                    row.get::<usize, String>(3).unwrap(),
                    row.get::<usize, i32>(4).unwrap())
                )
            })
    {
        Ok(user) => user,
        Err(error) => {return Err(format!("Unable to authenticate user: {}", error))}
    };
    
    if fetched_user.verify(&*credentials.password)
    {
        let result = fetched_user;
        return Ok(result)
    }
    
    Err("Unable to authenticate user: Incorrect password".to_string())
}