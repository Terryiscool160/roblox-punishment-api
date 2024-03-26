### Ban Storage

- `roblox_id`: BIGINT, NOT NULL, UNIQUE
  - Primary key
- `added`: TIMESTAMP, NOT NULL
  - Default: `CURRENT_TIMESTAMP`
- `updated`: TIMESTAMP, NOT NULL
  - Default: `CURRENT_TIMESTAMP`
- `unbanned_at`: BIGINT, NOT NULL - this should be os.time() + ban duration in seconds
- `countdown_start`: BIGINT, NOT NULL, DEFAULT 0 - should be set to os.time() + duration when the player joins the game
- `reason`: VARCHAT(1000), NOT NULL

```sql
CREATE TABLE IF NOT EXISTS bans (
  roblox_id BIGINT NOT NULL UNIQUE PRIMARY KEY,
  added TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  countdown_start BIGINT DEFAULT 0 NOT NULL,
  unbanned_at BIGINT NOT NULL,
  reason VARCHAR(1000) NOT NULL
);
```

### Log Storage

- `roblox_id`: BIGINT, NOT NULL
- `log_id`: VARCHAR(100), NOT NULL, UNIQUE
  - Primary key
- `added`: TIMESTAMP, NOT NULL
  - Default: `CURRENT_TIMESTAMP`
- `unbanned_at`: BIGINT, NOT NULL
- `duration`: VARCHAR(1000), NOT NULL
- `moderator`: VARCHAR(1000), NOT NULL
- `reason`: VARCHAR(1000), NOT NULL

```sql
CREATE TABLE IF NOT EXISTS logs (
  roblox_id BIGINT NOT NULL,
  log_id VARCHAR(100) UNIQUE PRIMARY KEY NOT NULL,
  added TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  unbanned_at BIGINT NOT NULL,
  duration VARCHAR(1000) NOT NULL,
  moderator VARCHAR(100) NOT NULL,
  reason VARCHAR(1000) NOT NULL
)
```
