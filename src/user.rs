use axum::extract::Path;
use axum::http::StatusCode;
use crate::database::CONN;

use axum::Json;
use rusqlite::named_params;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use sql_query_builder as sql;

#[derive(Deserialize, Serialize)]
pub struct User
{
    id: i32,
    username: String,
    email: String,
    points: i32
}

impl User
{
    pub fn new(
        id: i32,
        username: String,
        email: String,
        points: i32
    ) -> User
    {
        User
        {
            id,
            username,
            email,
            points
        }
    }
}

pub async fn index(path: Path<i32>) -> Result<Json<User>, (StatusCode, Json<String>)>
{
    let requested_user_id: i32 = path.0;
    let query: String = sql::Select::new()
        .select("id, name, email, points")
        .from("users")
        .where_clause("id = :user_id")
        .as_string();

    let conn = CONN.lock().unwrap();
    let stmt = conn.prepare(&query);

    let mut result = stmt.unwrap();

    let user = result.query_row(named_params! {":user_id": requested_user_id}, |row| {
        Ok(User::new(
            row.get::<usize, i32>(0).unwrap(),
            row.get::<usize, String>(1).unwrap(),
            row.get::<usize, String>(2).unwrap(),
            row.get::<usize, i32>(3).unwrap()
        ))
    });
        
    match user 
    {
        Ok(user) => Ok(Json(user)),
        Err(error) => Err((StatusCode::NOT_FOUND, Json(format!("Unable to query user: {}", error.to_string()))))
    }
}

pub async fn show() -> Result<Json<Vec<User>>, (StatusCode, Json<String>)>
{
    let query: String = sql::Select::new()
        .select("id, name, email, points")
        .from("users")
        .as_string();

    let conn = CONN.lock().unwrap();
    let stmt = conn.prepare(&query);

    let mut result = stmt.unwrap();

    let users = result.query_map([], |row| {
        Ok(User::new(
            row.get::<usize, i32>(0).unwrap(),
            row.get::<usize, String>(1).unwrap(),
            row.get::<usize, String>(2).unwrap(),
            row.get::<usize, i32>(3).unwrap()
        ))
    }).unwrap();
    // TODO: Fix this unwrap

    let mut user_vector: Vec<User> = Vec::new();

    for user in users
    {
        user_vector.push(user.unwrap());
    };
    
    Ok(Json(user_vector))
}

pub async fn store(body: String) -> Result<Json<String>, (StatusCode, Json<String>)>
{
    #[derive(Deserialize)]
    struct Request
    {
        username: String,
        email: String,
        password: String
    }

    let request: Request = match deserialize_request::<Request>(body.as_str()) {
        Ok(request) => request,
        Err(error) => {
            return Err((StatusCode::BAD_REQUEST, Json(format!("Unable to create new user: {}", error))));
        }
    };

    let encrypted_password: String = bcrypt::hash(request.password, 12).unwrap();

    let conn = CONN.lock().unwrap();

    let query: String = sql::Select::new()
        .select("COUNT(*)")
        .from("users")
        .where_clause("email = :email")
        .where_or("name = :username")
        .as_string();

    let stmt = conn.prepare(&*query);
    let has_taken_credentials: bool = stmt
        .unwrap()
        .query_row(named_params! {":email": request.email}, |row|
            {
                row.get::<usize, bool> (0)
            })
        .unwrap_or_else(|_| true);

    if has_taken_credentials
    {
        return Err((StatusCode::CONFLICT, Json("This email or username is already in use".to_string())))
    }

    let query: String = sql::Insert::new()
        .insert_into("users (name, email, password)")
        .values("(:username, :email, :password)")
        .to_string();
    
    let stmt= conn.prepare(&*query);

    match stmt
        .unwrap()
        .execute(named_params! {":username": request.username, ":email": request.email, ":password": encrypted_password})
    {
        Ok(_) => Ok(Json(format!("Successfully created a new user: {}", request.username))),
        Err(error) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Unable to create a new user: {}", error))))
    }
}

pub async fn update(path: Path<i32>) -> Result<Json<String>, (StatusCode, Json<String>)>
{
    let requested_user_id: i32 = path.0;

    Err((StatusCode::NOT_IMPLEMENTED, Json("Not implemented yet".to_string())))
}

pub async fn destroy(path: Path<i32>) -> Result<Json<String>, (StatusCode, Json<String>)>
{
    let requested_user_id: i32 = path.0;

    Err((StatusCode::NOT_IMPLEMENTED, Json("Not implemented yet".to_string())))
}

fn deserialize_request<T: DeserializeOwned>(body: &str) -> Result<T, String>
{
    match serde_json::from_str(body) {
        Ok(request) => Ok(request),
        Err(error) => Err(format!("Unable to create new user: {}", error)),
    }
}