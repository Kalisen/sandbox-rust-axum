use axum::{routing::{get, post}, Router};
use crate::state::AppState;
use super::handlers::*;

pub const USERS_ROUTE: &str = "/users";

pub trait AddUsersRoutes {
    fn add_users_routes(self) -> Router<AppState>;
}

impl AddUsersRoutes for Router<AppState> {
    fn add_users_routes(self) -> Router<AppState> {
        add_users_routes(self)
    }
}

fn add_users_routes(app_router: Router<AppState>) -> Router<AppState> {
    app_router.nest(
    USERS_ROUTE, 
    Router::new()
            .route("/", get(list_users))
            .route("/new", get(new_user))
            .route("/", post(create_user))
            .route("/:user_id", get(get_user))
    )
}