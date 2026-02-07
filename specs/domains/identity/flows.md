# Процессы Идентификации (Identity Flows)

## 1. Аутентификация и Синхронизация (Auth)

### 1.1 Вход
### 1.1 Вход и Данные (Auth & Data Fetching)
* **Метод**: Stateless Authentication (JWT).
* **Логика**:
    1. Клиент авторизуется через виджет и получает JWT токен от Бэкенда Mentor.
    2. В токене зашит `telegram_user_id` (ключ к Network).
    3. Клиент передает заголовок `Authorization: Bearer <token>`.
    4. **Запрос профиля (`GetProfile`)**:
        * Mentor API проверяет подпись JWT.
        * Mentor API идет в **Network Service** (Real-time) с `telegram_user_id`.
        * Получает: `username`, `avatar`, `balances`.
        * Обогащает ответ локальными настройками (`User.settings`).
        * Возвращает агрегированный `UserProfile`.
    5. **Регистрация (Lazy)**:
        * Если при запросе локальный `User` не найден -> создается "на лету" с дефолтными настройками.
* **Важно**: Мы НЕ храним `username`, `avatar`, `balance` в БД Mentor. Они всегда свежие из Network.

### 1.3 Real-time Updates (Streaming)
* **Метод**: `SubscribeToAccountUpdates(Empty) -> stream AccountUpdate`.
* **Назначение**: Вместо Push-уведомлений и поллинга, клиент держит постоянное соединение.
* **События в стриме**:
    * `BalanceChanged`: Изменился баланс в Network (пришло пополнение).
    * `QuotaChanged`: Изменились остатки лимитов (потратил сообщение).
    * `SubscriptionChanged`: Подписка продлилась или истекла.
* **Использование**: Фронтенд слушает стрим и обновляет UI реактивно.
