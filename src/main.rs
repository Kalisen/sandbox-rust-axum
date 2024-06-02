use chrono;
use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};
use state::{AppState, RendererState};
use tokio::{self, sync::Mutex};
use axum::{http::StatusCode, routing::get, Router};
use users::{AddUsersRoutes, User};
use std::{collections::HashMap, sync::Arc};

mod users;
mod state;

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
    .route("/", get(|| async { "Hello, World!" }))
    .add_users_routes()
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

fn now_helper(_: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    let now = chrono::offset::Local::now().to_string();
    out.write(&now)?;
    Ok(())
}
