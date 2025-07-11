use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};
use serde::Deserialize;
use uuid::Uuid;

use crate::AppState;
use crate::models::contact::{self, Entity as ContactEntity};

#[derive(Deserialize)]
pub struct UpdateContact {
    pub email: Option<String>,
}

pub async fn list_contacts(
    State(state): State<AppState>,
) -> Result<Json<Vec<contact::Model>>, (StatusCode, String)> {
    let contacts = ContactEntity::find().all(&*state.db_conn).await;

    match contacts {
        Ok(contacts) => Ok(Json(contacts)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_contact(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<contact::Model>, (StatusCode, String)> {
    let contact = ContactEntity::find_by_id(id).one(&*state.db_conn).await;

    match contact {
        Ok(Some(contact)) => Ok(Json(contact)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Contact not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn update_contact(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateContact>,
) -> Result<Json<contact::Model>, (StatusCode, String)> {
    let contact_to_update = ContactEntity::find_by_id(id)
        .one(&*state.db_conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Contact not found".to_string()))?;
    let mut active_contact = contact_to_update.into_active_model();

    if let Some(email) = payload.email {
        active_contact.email = Set(email);
    }

    let updated_contact = active_contact.update(&*state.db_conn).await;

    match updated_contact {
        Ok(contact) => Ok(Json(contact)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn delete_contact(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = ContactEntity::delete_by_id(id).exec(&*state.db_conn).await;

    match result {
        Ok(delete_result) => {
            if delete_result.rows_affected == 1 {
                Ok(StatusCode::NO_CONTENT)
            } else {
                Err((StatusCode::NOT_FOUND, "Contact not found".to_string()))
            }
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
