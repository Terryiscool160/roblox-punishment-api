# current api endpoints

## punishing a user:

```bash
curl -d '{"roblox_id": 1, "reason":"chat bypassing", "username":"ROBLOX", "unbanned_at"}' -H "Content-type: application/json" -X POST http://127.0.0.1:8080/Punish
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
  "username": "unknodsdfsdwn",
  "reason": "smdsfsdells"
}
```

```json
{
  "message": "NotFound"
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
    "username": "unknodsdfsdwn",
    "reason": "smdsfsdells"
  }
]
```
