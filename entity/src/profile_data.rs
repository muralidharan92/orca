//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ProfileData {
    pub name: String,
    pub value: String
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, DeriveIntoActiveModel)]
pub struct EnvironmentData {
    pub name: String,
    pub value: String
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "profile_data")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub name: String,
    pub value: String,
    pub profile_id: i32
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::profile::Entity",
        from = "Column::ProfileId",
        to = "super::profile::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Profile
}


impl Related<super::profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Profile.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
