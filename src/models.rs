use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::bans)]
pub struct Ban {
    pub roblox_id: i64,
    pub added: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub unbanned_at: i64,
    pub username: String,
    pub reason: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub updated: String
}

#[derive(Debug, Deserialize)]
pub struct NewBanJSON {
    pub roblox_id: i64,
    pub unbanned_at: i64,
    pub username: String,
    pub reason: String,
}