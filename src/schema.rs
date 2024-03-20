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
