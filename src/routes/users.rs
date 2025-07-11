use crate::{
    AppState,
    handlers::users::{create_user_and_contact, list_users, get_user},
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/users",
            post(create_user_and_contact).get(list_users),
        )
        .route("/users/:id", get(get_user))
}
