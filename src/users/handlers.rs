use axum::{extract::{Path, State}, response::{Html, IntoResponse, Redirect}, Form, Json};
use axum_macros::debug_handler;
use serde_json::json;

use crate::state::{AppState, RendererState};

use super::{User, UserForm, USERS_ROUTE};

pub async fn new_user(State(state): State<RendererState>) -> impl IntoResponse {
    match state.registry.render("new_user.hbs", &"{}") {
        Ok(content) => Html(content).into_response(),
        Err(error) => error.reason().to_string().into_response()
    }
}

pub async fn list_users(State(state): State<AppState>) -> impl IntoResponse {
    let users = state.users.lock().await;
    let users_list: Vec<User> = users.iter().map(|(_,v)| v.clone()).collect();
    match state.renderer_state.registry.render("list_users.hbs", &json!({"user": users_list})) {
        Ok(content) => Html(content).into_response(),
        Err(error) => error.reason().to_string().into_response()
    }
}

pub async fn get_user(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let users = state.users.lock().await;
    match users.get(&id) {
        Some(user) => Json(user.clone()).into_response(),
        None => Redirect::to(USERS_ROUTE).into_response()
    }
}

#[debug_handler]
pub async fn create_user(State(state): State<AppState>, Form(user_form): Form<UserForm>) -> Redirect {
    let user = User {
        id: rand::random(),
        first_name: user_form.first_name,
        last_name: user_form.last_name,
        occupation: user_form.occupation
    };
    let mut users = state.users.lock().await;
    users.insert(user.id, user);
    Redirect::to(USERS_ROUTE)
}

