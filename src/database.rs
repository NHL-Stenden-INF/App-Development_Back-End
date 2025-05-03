use std::sync::Mutex;
use lazy_static;
use rusqlite;
use rusqlite::Connection;

lazy_static::lazy_static!
{
    pub static ref CONN: Mutex<Connection> = Mutex::new(Connection::open("app-dev.db").unwrap());
}