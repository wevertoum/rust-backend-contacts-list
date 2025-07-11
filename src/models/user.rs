use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Char(Some(1))")]
pub enum Genre {
    #[sea_orm(string_value = "M")]
    Male,
    #[sea_orm(string_value = "F")]
    Female,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub genre: Genre,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::contact::Entity")]
    Contact,
}

impl Related<super::contact::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contact.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
