//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sea_orm::EntityTrait;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "audit_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub event: String,
    pub action: String,
    pub payload: String,
    pub acted_by: i64,
    pub created_at: DateTimeUtc
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user::Entity")]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
