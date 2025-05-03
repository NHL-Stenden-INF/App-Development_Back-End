use std::string::ToString;
use std::sync::Mutex;
use lazy_static;
use rusqlite;
use rusqlite::{named_params, Connection, Statement};
use sql_query_builder as sql;

lazy_static::lazy_static!
{
    pub static ref CONN: Mutex<Connection> = Mutex::new(Connection::open("app-dev.db").unwrap());
}

pub fn add_user(username: &str, email: &str, raw_password: &str) -> Result<String, String>
{
    let encrypted_password: String = bcrypt::hash(raw_password, 12).unwrap();

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
        .query_row(named_params! {":email": email}, |row|
            {
                row.get::<usize, bool> (0)
            })
        .unwrap_or_else(|_| true);

    if has_taken_credentials
    {
        return Err("This email or username is already in use".to_string())
    }

    let query: String = sql::Insert::new()
        .insert_into("users (name, email, password)")
        .values("(:username, :email, :password)")
        .to_string();
    
    
    let stmt= conn.prepare(&*query);

    let _ = stmt
        .unwrap()
        .execute(named_params! {":username": username, ":email": email, ":password": encrypted_password});
    
    Ok(format!("Successfully created a new user: {}", username))
}