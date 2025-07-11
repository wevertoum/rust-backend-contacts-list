use crate::{
    handlers::users::{
        create_user_and_contact, delete_user, get_user, list_users, update_user,
    },
    AppState,
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(create_user_and_contact).get(list_users))
        .route(
            "/users/:id",
            get(get_user).put(update_user).delete(delete_user), // Adiciona PUT e DELETE
        )
}