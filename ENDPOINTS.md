# current api endpoints

## punishing a user:

```bash
curl -d '{"roblox_id": 1, "duration": "1 week", "reason":"chat bypassing", "unbanned_at": 0, "moderator":"terryiscool160"}' -H "Content-type: application/json" -H "Authorization: yourmom" -X POST http://127.0.0.1:8080/Punish
```

example response:

```json
{
  "success": true
}
```

## getting a user's punishment:

```bash
curl -H "Authorization: yourmom" -X GET http://127.0.0.1:8080/Punishment/{userId}
```

example responses:

```json
{
  "roblox_id": 3,
  "added": "2024-03-10T19:08:52.174234",
  "updated": "2024-03-10T19:08:52.174693",
  "unbanned_at": 1710097709,
  "reason": "smdsfsdells"
}
```

```json
{
  "message": "Not found within database"
}
```

## getting a user's past punishments:

```bash
curl -H "Authorization: yourmom" -X GET http://127.0.0.1:8080/Logs/{userId}
```

example response:

```json
[
  {
    "roblox_id": 1,
    "log_id": "1wheM",
    "added": "2024-03-24T14:25:15.777033",
    "unbanned_at": 0,
    "duration": "1 week",
    "reason": "chat bypassing",
    "moderator": "terryiscool160"
  }
]
```

## removing a user's past punishments:

```bash
curl -H "Authorization: yourmom" -X POST http://127.0.0.1:8080/RemoveLog/{log_id}
```

example response:

```json
{
  "success": true
}
```

## appealing a user's punishment:

```bash
curl -H "Content-type: application/json" -H "Authorization: yourmom" -X POST http://127.0.0.1:8080/Appeal/{userId}
```

example response:

```json
{
  "success": true
}
```

## getting all punishments:

```bash
curl -H "Authorization: yourmom" -X GET http://127.0.0.1:8080/Punishments
```

example response:

```json
[
  {
    "roblox_id": 3,
    "added": "2024-03-10T19:08:52.174234",
    "updated": "2024-03-10T19:08:52.174693",
    "unbanned_at": 1710097709,
    "reason": "smdsfsdells"
  }
]
```
