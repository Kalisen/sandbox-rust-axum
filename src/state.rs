use axum::extract::FromRef;
use handlebars::{self, Handlebars};
use tokio::{self, sync::Mutex};
use std::{collections::HashMap, sync::Arc};
use crate::users::User;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<HashMap<i32, User>>>,
    pub renderer_state: RendererState
}

#[derive(Clone)]
pub struct RendererState {
    pub registry: Handlebars<'static>
}

impl FromRef<AppState> for RendererState {
    fn from_ref(state: &AppState) -> RendererState {
        state.renderer_state.clone()    
    }
}
