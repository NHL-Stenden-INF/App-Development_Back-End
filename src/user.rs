use crate::response;
use crate::database::CONN;

use axum::Json;
use axum::response::Html;
use rusqlite::named_params;
use serde::{Serialize, Deserialize};
use sql_query_builder as sql;

#[derive(Serialize)]
struct User
{
    id: i32,
    username: String,
    email: String,
    password: String,
    points: i32
}

pub async fn index(body: String) -> Html<&'static str>
{
    Html("<h1>Hello, world!</h1>")
}

pub async fn show()
{

}

pub async fn store(body: String) -> Result<Json<response::Response>, Json<response::Response>>
{
    #[derive(Deserialize)]
    struct Request
    {
        username: String,
        email: String,
        password: String
    }
    
    let request: Request = match serde_json::from_str(&body)
    {
        Ok(request) => request,
        Err(error) => 
            {
                return Err(response::Response::new(&format!("Unable to create new user: {}", error))
                    .to_json())
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
        return Err(response::Response::new("This email or username is already in use")
                .to_json())
    }

    let query: String = sql::Insert::new()
        .insert_into("users (name, email, password)")
        .values("(:username, :email, :password)")
        .to_string();


    let stmt= conn.prepare(&*query);

    let _ = stmt
        .unwrap()
        .execute(named_params! {":username": request.username, ":email": request.email, ":password": encrypted_password});

    Ok(response::Response::new(&format!("Successfully created a new user: {}", request.username))
           .to_json())
}

pub async fn update()
{
    
}

pub async fn destroy()
{
    
}