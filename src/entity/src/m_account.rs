//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Default,DeriveEntityModel, Eq,Deserialize, Serialize)]
#[sea_orm(table_name = "m_account")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub account_name: Option<String>,
    pub password: Option<String>,
    pub current_token: Option<String>,
    #[sea_orm(column_name = "token_deadLine")]
    pub token_dead_line: Option<DateTime>,
    pub active: i32,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
