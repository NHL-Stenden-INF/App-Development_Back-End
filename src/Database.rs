use std::string::ToString;
use std::sync::{LockResult, Mutex};
use bcrypt::{BcryptResult, HashParts};
use lazy_static;
use rusqlite;
use rusqlite::{named_params, params, Connection};

lazy_static::lazy_static!
{
    static ref CONN: Mutex<Connection> = Mutex::new(Connection::open("app-dev.db").unwrap());
}

static GET_USER_QUERY: &str = "SELECT * FROM users WHERE `email` = :email;";
static ADD_USER_QUERY: &str = "INSERT INTO users (`name`, `email`, `password`) VALUES (:name, :email, :password);";

pub fn validate_user(email: &str, password: &str) -> bool
{
    let conn = CONN.lock().unwrap();
    
    let stmt = conn.prepare(GET_USER_QUERY);
    let encrypted_password = stmt
        .unwrap()
        .query_row(named_params! {":email": email}, |row| 
            {
                row.get::<usize, String> (3)
            })
        .unwrap_or_else(|error| {error.to_string()});
    
    bcrypt::verify(password, &*encrypted_password).unwrap()
}

pub fn add_user(username: &str, email: &str, raw_password: &str)
{
    let encrypted_password: String = bcrypt::hash(raw_password, 12).unwrap();

    let conn = CONN.lock().unwrap();

    let mut stmt= conn.prepare(GET_USER_QUERY);
    
 
    stmt = conn.prepare(ADD_USER_QUERY);
    let _ = stmt.unwrap().execute(named_params! {":name": username, ":email": email, ":password": encrypted_password});
}