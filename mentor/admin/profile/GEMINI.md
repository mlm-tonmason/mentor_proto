# mentor/admin/profile/

## ProfileAdminService

Админ-сервис управления профилями пользователей — инструменты поддержки и расследований.

### RPC-методы

| Метод | Описание |
|-------|----------|
| `GetUserProfile` | Полный профиль пользователя с `max_bots`, количеством ботов и подписок |
| `SearchUsers` | Поиск по `telegram_id` или `telegram_username` |

### Модели

- **`UserProfileResponse`** — агрегация: `UserProfile` + `max_bots` + `current_bot_count` + `active_subscriptions_count`
- **`SearchUsersRequest`** — приоритет поиска: `telegram_id` > `telegram_username`

### Особенности

- Заменяет старый `ProfileAdminService` с шаблонами (отложены до будущей фичи).
- Доступ: только администратор.
