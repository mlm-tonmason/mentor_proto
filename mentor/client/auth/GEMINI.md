# mentor/client/auth/

## AuthService

Сервис аутентификации и управления сессиями.

### RPC-методы

| Метод | Описание |
|-------|----------|
| `Login` | Вход через Telegram (Mini App или Widget). Создаёт/возвращает сессию |
| `Logout` | Завершение сессии, деактивация токена |

### Модели

- **`LoginRequest`** — `oneof method { TelegramMiniAppAuth, TelegramWidgetAuth }`
- **`TelegramMiniAppAuth`** — авторизация через `initData` от TWA
- **`TelegramWidgetAuth`** — авторизация через Telegram Login Widget с `ref_link_id`
- **`LoginResponse`** — `session_token` + `UserProfile`

### Особенности

- Единственный сервис с **публичными методами** (не требуют `Authorization`).
- Ленивая регистрация: при первом входе создаётся пользователь с дефолтными настройками.
- Реферальная ссылка берётся из `start_param` (Mini App) или `ref_link_id` (Widget).
