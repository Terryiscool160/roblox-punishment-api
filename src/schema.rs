// @generated automatically by Diesel CLI.

diesel::table! {
    bans (roblox_id) {
        roblox_id -> BigInt,
        added -> Timestamp,
        updated -> Timestamp,
        unbanned_at -> BigInt,
        reason -> Text,
    }
}

diesel::table! {
    logs (log_id) {
        roblox_id -> BigInt,
        log_id -> Text,
        added -> Timestamp,
        unbanned_at -> BigInt,
        duration -> Text,
        moderator -> Text,
        reason -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bans,
    logs,
);
