use axum_macros::debug_handler;
use chrono;
use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};
use tokio::{self, sync::Mutex};
use axum::{extract::{FromRef, Path, State}, http::StatusCode, response::{Html, IntoResponse, Redirect}, routing::{get, post}, Form, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
struct AppState {
    users: Arc<Mutex<HashMap<i32, User>>>,
    renderer_state: RendererState
}

#[derive(Clone)]
struct RendererState {
    registry: Handlebars<'static>
}

impl FromRef<AppState> for RendererState {
    fn from_ref(state: &AppState) -> RendererState {
        state.renderer_state.clone()    
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct User {
    id: i32,
    first_name: String,
    last_name: String,
    occupation: String
}

#[derive(Deserialize)]
struct UserForm {
    first_name: String,
    last_name: String,
    occupation: String
}

// Take a URL extract the content
// pass it to an LLM for Summary and post reference post to reddit
#[tokio::main]
async fn main() {
    //setup handlebars
    let mut registry: Handlebars<'static> = Handlebars::new();
    registry.register_template_file("new_user.hbs", "templates/new_user.hbs").unwrap();
    registry.register_template_file("list_users.hbs", "templates/list_users.hbs").unwrap();
    registry.register_helper("now", Box::new(now_helper));

    let users: HashMap<i32, User> = HashMap::new();
    let app_state = AppState {
        users: Arc::new(Mutex::new(users)),
        renderer_state: RendererState {
            registry
        }
    };

    // build our application with a single route
    let app = Router::new()
    .route("/1", get(handler_1))
    .route("/ok", get(ok))
    .route("/user", get(list_users))
    .route("/user/new", get(new_user))
    .route("/user", post(create_user))
    .route("/user/:user_id", get(get_user))
    .route("/", get(|| async { "Hello, World!" }))
    .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler_1() -> &'static str {
    "One!"
}

async fn ok() -> StatusCode {
    StatusCode::OK
}

async fn new_user(State(state): State<RendererState>) -> impl IntoResponse {
    match state.registry.render("new_user.hbs", &"") {
        Ok(content) => Html(content).into_response(),
        Err(error) => error.reason().to_string().into_response()
    }
}

async fn list_users(State(state): State<AppState>) -> impl IntoResponse {
    let users = state.users.lock().await;
    let users_list: Vec<User> = users.iter().map(|(_,v)| v.clone()).collect();
    match state.renderer_state.registry.render("list_users.hbs", &json!({"user": users_list})) {
        Ok(content) => Html(content).into_response(),
        Err(error) => error.reason().to_string().into_response()
    }
}

async fn get_user(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let users = state.users.lock().await;
    match users.get(&id) {
        Some(user) => Json(user.clone()).into_response(),
        None => Redirect::to("/user").into_response()
    }
}

#[debug_handler]
async fn create_user(State(state): State<AppState>, Form(user_form): Form<UserForm>) -> Redirect {
    let user = User {
        id: rand::random(),
        first_name: user_form.first_name,
        last_name: user_form.last_name,
        occupation: user_form.occupation
    };
    let mut users = state.users.lock().await;
    users.insert(user.id, user);
    Redirect::to("/user")
}

fn now_helper(_: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    let now = chrono::offset::Local::now().to_string();
    out.write(&now)?;
    Ok(())
}
