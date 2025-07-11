use crate::{
    AppState,
    handlers::contacts::{get_contact, list_contacts, update_contact},
};
use axum::{Router, routing::get};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/contacts", get(list_contacts))
        .route("/contacts/:id", get(get_contact).put(update_contact))
}
