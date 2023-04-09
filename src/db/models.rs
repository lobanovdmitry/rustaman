use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(
    Debug, Insertable, Queryable, Identifiable, Serialize, Deserialize, PartialEq, Selectable,
)]
#[diesel(table_name = super::schema::app_user)]
pub struct AppUser {
    pub id: i64,
    pub username: String,
    pub updated_at: Option<SystemTime>,
}
