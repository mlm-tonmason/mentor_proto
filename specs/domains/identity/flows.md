# Процессы Идентификации (Identity Flows)

## 1. Аутентификация и Синхронизация (Auth)

### 1.1 Вход
### 1.1 Вход и Данные (Auth & Data Fetching)
* **Метод**: Stateful Session Authentication (Opaque Token).
* **Логика**:
    1. **Авторизация (`AuthService.Login`)**:
        * **Mini App**: Клиент передает `init_data` (с user, hash, start_param) и `bot_id`. Сервер валидирует подпись ключом бота. Реферальная ссылка берется из `start_param`.
        * **Widget**: Клиент передает данные виджета (`data`), `bot_id` и явно `ref_link_id` (так как виджет не передает рефералку).
    2. **Сессия**:
        * Бэкенд создает (или находит) активную сессию в Redis/DB.
        * Возвращает постоянный `session_token` (Long-lived).
    3. **Доступ к API**:
        * Клиент передает заголовок `Authorization: Bearer <session_token>`.
        * Бэкенд проверяет валидность сессии по токену.
    4. **Запрос профиля (`GetProfile`)**:
        * Ментор API идет в **Network Service** (Real-time sync) с ID пользователя из сессии.
        * Получает: `username`, `avatar`, `balances`.
        * Обогащает ответ локальными настройками (`User.settings`).
        * Возвращает агрегированный `UserProfile`.
    5. **Регистрация (Lazy)**:
        * Если при входе локальный `User` не найден -> создается "на лету" с дефолтными настройками и привязкой реферала.
* **Важно**: Мы НЕ храним `username`, `avatar`, `balance` в БД Mentor. Они всегда свежие из Network.

### 1.3 Real-time Updates (Streaming)
* **Метод**: `SubscribeToAccountUpdates(Empty) -> stream AccountUpdate`.
* **Назначение**: Вместо Push-уведомлений и поллинга, клиент держит постоянное соединение.
* **События в стриме**:
    * `BalanceChanged`: Изменился баланс в Network (пришло пополнение).
    * `QuotaChanged`: Изменились остатки лимитов (потратил сообщение).
    * `SubscriptionChanged`: Подписка продлилась или истекла.
* **Использование**: Фронтенд слушает стрим и обновляет UI реактивно.
