//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(column_type = "VarBinary(StringLen::None)")]
    pub selector: Vec<u8>,
    pub name: String,
    #[sea_orm(column_type = "JsonBinary")]
    pub inputs: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
