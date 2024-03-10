CREATE TABLE IF NOT EXISTS bans (
  roblox_id BIGINT NOT NULL UNIQUE PRIMARY KEY,
  added TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  unbanned_at BIGINT NOT NULL,
  username VARCHAR(20) NOT NULL UNIQUE,
  reason VARCHAR(1000) NOT NULL
)