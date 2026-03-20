# mentor/admin/

Административные gRPC-сервисы — API для управления платформой.

## Сервисы

| Директория | Сервис | Назначение |
|------------|--------|------------|
| `author/` | `AuthorAdminService` | Управление лимитами авторов (max_bots) |
| `bot/` | `BotAdminService` | CRUD ботов, статусы, тренды, тарифы, квоты, привязки |
| `profile/` | `ProfileAdminService` | Профили пользователей для поддержки, поиск по Telegram |
| `subscription/` | `SubscriptionAdminService` | Подарочные подписки, продление, отмена, просмотр |
| `voice/` | `VoiceAdminService` | Управление голосами (stub, ожидает интеграции VAPI) |

## Авторизация

Два уровня доступа:
1. **Создатель сущности** — `user_id == entity.creator_id`
2. **Администратор** — `admin_user_ids.contains(user_id)`

Admin user IDs задаются через CLI: `--admin-ids 1,2,3`.
