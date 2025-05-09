mod user;
mod database;
mod auth;
mod game;
mod friends;

use axum;
use axum::response::Html;
use axum::{middleware, Router};
use axum::routing::{any, get, post, patch, delete};

const HOST: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main()
{
    println!("Started server on: {}", HOST);
    // Routes here will be protected by the middleware
    let protected_routes = Router::new()
        // User routes
        .route("/user/", get(user::show))
        .route("/user/{user-id}", get(user::index))
        .route("/user/{user-id}", patch(user::update))
        .route("/user/{user-id}", delete(user::destroy))
        // Game routes
        .route("/game/", get(game::show))
        .route("/game/{game-id}", get(game::index))
        .route("/game/{game-id}", patch(game::update))
        .route("/game/{game-id}/user/{user-id}", get(game::index_game_by_user))
        .route("/game/{game-id}/user/{user-id}", patch(game::update_game_by_user))
        // Friends routes
        .route("/friend/", get(friends::show))
        .route("/friend/{user-id}", get(friends::index))
        .route("/friend/{user-id}/add/{friend-id}", post(friends::store))
        .route("/friend/{user-id}/remove/{friend-id}", post(friends::destroy))
        .layer(middleware::from_fn(auth::authenticate));
    
    // Routes here will not
    let unprotected_routes = Router::new()
        // Root route (unused)
        .route("/", any(root))
        // User routes
        .route("/user/", post(user::store))
        .route("/auth/", get(auth::get_user_from_header_json));
    
    let app = Router::new()
        .merge(protected_routes)
        .merge(unprotected_routes);
    

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(HOST)
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
