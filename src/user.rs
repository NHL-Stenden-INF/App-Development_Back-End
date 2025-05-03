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
    password: String,
    points: i32
}

pub async fn index(body: String) -> Result<Json<String>, Json<String>>
{
    Ok(Json("Not implemented yet".to_string()))
}

pub async fn show(body: String)  -> Result<Json<Vec<User>>, Json<String>>
{
    let query: String = sql::Select::new()
        .select("*")
        .from("users")
        .as_string();

    let conn = CONN.lock().unwrap();
    let stmt = conn.prepare(&query);

    let mut result = stmt.unwrap();

    let users = result.query_map([], |row| {
        Ok(User {
            id: row.get::<usize, i32>(0).unwrap(),
            username: row.get::<usize, String>(1).unwrap(),
            email: row.get::<usize, String>(2).unwrap(),
            password: row.get::<usize, String>(3).unwrap(),
            points: row.get::<usize, i32>(4).unwrap()
        })
    }).unwrap();

    let mut user_vector: Vec<User> = Vec::new();

    for user in users
    {
        user_vector.push(user.unwrap());
    };
    
    Ok(Json(user_vector))
}

pub async fn store(body: String) -> Result<Json<String>, Json<String>>
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
            return Err(Json(format!("Unable to create new user: {}", error)));
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
        return Err(Json("This email or username is already in use".to_string()))
    }

    let query: String = sql::Insert::new()
        .insert_into("users (name, email, password)")
        .values("(:username, :email, :password)")
        .to_string();


    let stmt= conn.prepare(&*query);

    let _ = stmt
        .unwrap()
        .execute(named_params! {":username": request.username, ":email": request.email, ":password": encrypted_password});

    Ok(Json(format!("Successfully created a new user: {}", request.username)))
}

pub async fn update() -> Result<Json<String>, Json<String>>
{
    Ok(Json("Not implemented yet".to_string()))
}

pub async fn destroy() -> Result<Json<String>, Json<String>>
{
    Ok(Json("Not implemented yet".to_string()))
}

fn deserialize_request<T: DeserializeOwned>(body: &str) -> Result<T, String>
{
    match serde_json::from_str(body) {
        Ok(request) => Ok(request),
        Err(error) => Err(format!("Unable to create new user: {}", error)),
    }
}