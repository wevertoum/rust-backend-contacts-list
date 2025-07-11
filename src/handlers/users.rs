use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set, TransactionTrait};
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize)]
pub struct UpdateUserPayload {
    pub name: Option<String>,
    pub genre: Option<user::Genre>,
    pub email: Option<String>,
}

#[derive(Serialize)]
pub struct UserWithContactResponse {
    pub id: Uuid,
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
) -> Result<Json<UserWithContactResponse>, (StatusCode, String)> {
    let user_and_contact = UserEntity::find_by_id(id)
        .find_also_related(contact::Entity)
        .one(&*state.db_conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match user_and_contact {
        Some((user, Some(contact))) => {
            let response = UserWithContactResponse {
                id: user.id,
                name: user.name,
                genre: user.genre,
                email: contact.email,
            };
            Ok(Json(response))
        }
        Some((_, None)) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "User found but related contact is missing.".to_string(),
        )),
        None => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    }
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<Json<UserWithContactResponse>, (StatusCode, String)> {
    let txn = state
        .db_conn
        .begin()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let (user_to_update, contact_to_update) = UserEntity::find_by_id(id)
        .find_also_related(contact::Entity)
        .one(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let contact_to_update = contact_to_update.ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Contact data missing".to_string(),
        )
    })?;

    let mut user_active_model = user_to_update.clone().into_active_model();
    if let Some(name) = payload.name {
        user_active_model.name = Set(name);
    }
    if let Some(genre) = payload.genre {
        user_active_model.genre = Set(genre);
    }
    let updated_user = user_active_model
        .update(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut contact_active_model = contact_to_update.into_active_model();
    if let Some(email) = payload.email {
        contact_active_model.email = Set(email);
    }
    let updated_contact = contact_active_model
        .update(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    txn.commit()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = UserWithContactResponse {
        id: updated_user.id,
        name: updated_user.name,
        genre: updated_user.genre,
        email: updated_contact.email,
    };

    Ok(Json(response))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = UserEntity::delete_by_id(id).exec(&*state.db_conn).await;

    match result {
        Ok(delete_result) => {
            if delete_result.rows_affected >= 1 {
                Ok(StatusCode::NO_CONTENT)
            } else {
                Err((StatusCode::NOT_FOUND, "User not found".to_string()))
            }
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
