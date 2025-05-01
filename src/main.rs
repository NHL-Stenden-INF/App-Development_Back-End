mod User;
mod Database;
mod Middleware;
mod Game;
mod Friends;

use std::io::ErrorKind::AddrInUse;
use axum;
use axum::response::Html;
use axum::{middleware, Router};
use axum::routing::{any, get, post, patch, delete};
use serde_json;
use serde_json::json;

#[tokio::main]
async fn main()
{
    // build our application with a single route
    let app = Router::new()
        // Root route (unused)
        .route("/", any(root))
        // User routes
        .route("/user", get(User::show))
        .route("/user", post(User::store))
        .route("/user/{user-id}", get(User::index))
        .route("/user/{user-id}", patch(User::update))
        .route("/user/{user-id}", delete(User::destroy))
        // Game routes
        .route("/game/", get(Game::show))
        .route("/game/{game-id}", get(Game::index))
        .route("/game/{game-id}", patch(Game::update))
        .route("/game/{game-id}/user/{user-id}", get(Game::index_game_by_user))
        .route("/game/{game-id)/user/{user-id}", patch(Game::update_game_by_user))
        // Friends routes
        .route("/friend/", get(Friends::show))
        .route("/friend/{user-id}", get(Friends::index))
        .route("/friend/{user-id}/add/{friend-id}", post(Friends::store))
        .route("/friend/{user-id{/remove/{friend-id}", post(Friends::destroy))
        .layer(middleware::from_fn(Middleware::authenticate));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn root() -> Html<&'static str>
{
    Html("<h1>Hi! This is an API for a school project. Feel free to snoop around, because it's all fake anyways :3</h1>\
    <a href=\"https://www.youtube.com/watch?v=dQw4w9WgXcQ\">Nice video</a>\
    <style>body {background-color:black;} h1 {color:white;}</style>")
}
