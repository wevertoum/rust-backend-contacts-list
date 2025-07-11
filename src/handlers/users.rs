use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TransactionTrait};
use serde::Deserialize;
use uuid::Uuid;

use crate::AppState;
use crate::models::contact;
use crate::models::user::{self, Entity as UserEntity};

#[derive(Deserialize)]
pub struct CreateUserWithContact {
    pub name: String,
    pub genre: user::Genre,
    pub email: String,
}

pub async fn create_user_and_contact(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserWithContact>,
) -> Result<Json<user::Model>, (StatusCode, String)> {
    let txn = state
        .db_conn
        .begin()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let new_user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(payload.name),
        genre: Set(payload.genre),
    };

    let user_model = new_user
        .insert(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let new_contact = contact::ActiveModel {
        id: Set(Uuid::new_v4()),
        email: Set(payload.email),
        user_id: Set(user_model.id),
    };
    new_contact
        .insert(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    txn.commit()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(user_model))
}

pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<user::Model>>, (StatusCode, String)> {
    let users = UserEntity::find().all(&*state.db_conn).await;

    match users {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<Json<user::Model>, (StatusCode, String)> {
    let user = UserEntity::find_by_id(id).one(&*state.db_conn).await;

    match user {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}