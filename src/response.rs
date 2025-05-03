use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response
{
    message: String,
}

impl Response
{
    pub fn new(message: &str) -> Response
    {
        Response
        {
            message: message.to_string(),
        }
    }

    pub fn to_json(self) -> Json<Response>
    {
        Json(self)
    }
}

#[derive(Serialize)]
pub struct ResponseWithList
{
    message: Vec<String>,
}

impl ResponseWithList
{
    pub fn new(message: Vec<String>) -> ResponseWithList
    {
        ResponseWithList
        {
            message
        }
    }

    pub fn to_json(self) -> Json<ResponseWithList>
    {
        Json(self)
    }
}