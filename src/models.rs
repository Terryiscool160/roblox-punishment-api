use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::bans)]
pub struct Ban {
    pub roblox_id: i64,
    pub added: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub countdown_start: i64,
    pub unbanned_at: i64,
    pub reason: String,
    pub log_id: String,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::logs)]
pub struct Log {
    pub roblox_id: i64,
    pub log_id: String,
    pub added: NaiveDateTime,
    pub unbanned_at: i64,
    pub duration: String,
    pub reason: String,
    pub moderator: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub updated: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NewBanJSON {
    pub roblox_id: i64,
    pub unbanned_at: i64,
    pub reason: String,
    pub moderator: String,
    pub duration: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StartCountdownJSON {
    pub roblox_id: i64,
    pub countdown_start: i64,
}
