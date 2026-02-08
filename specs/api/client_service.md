# Спецификация Клиентского API

Определения RPC, соответствующие текущей реализации в .proto файлах.

## 1. Сервис Аутентификации (Auth Service)
* `Login`: Вход в систему (Telegram Mini App / Widget). Возвращает токен сессии.
* `Logout`: Завершение сессии.
* `RefreshToken`: Обновление токена доступа.

## 2. Сервис Идентификации (Identity Service)
* `GetMyProfile`: Получить "себя" (детализированный профиль с балансами и настройками).
* `ChangeLanguage`: Сменить язык коммуникации с ботами.

## 3. Сервис Маркета (Market Service)
* `GetBot`: Получить детальную информацию о боте.
* `SearchBots`: Поиск и фильтрация ботов (Каталог).
* `RateBot`: Оценить бота.
* `AddToFavorites` / `RemoveFromFavorites`: Управление избранным.
* `ListMyLibrary`: Моя библиотека (купленные, избранные, недавние).

## 4. Сервис Биллинга (Billing Service)
* `GetSubscriptionDashboard`: Дашборд подписок (активные, архивные, рекомендованные).
* `GetTransactions`: История финансовых операций.
* `GetTransaction`: Детали конкретной транзакции.

## 5. Сервис Чата (Chat Service)
### Контакты и Боты
* `ListContactBots`: Список ботов-контактов (активные диалоги, закрепленные).
* `PinBot` / `UnpinBot`: Закрепление бота в списке контактов.
* `ReorderPinnedBots`: Изменение порядка закрепленных ботов.

### Диалоги
* `ListChats`: Список диалогов с конкретным ботом.
* `UpdateChatTitle`: Переименовать чат.
* `UpdateChatConfiguration`: Обновить настройки чата (режим ответа, thinking mode).
* `DeleteChat`: Удалить диалог.

### Сообщения
* `GetHistory`: Получить историю сообщений (с фильтром по избранным).
* `SendMessage`: Отправить сообщение (текст/голос).
* `AddToFavorites` / `RemoveFromFavorites`: Избранные сообщения.
* `SubscribeToEvents`: Real-time стрим событий (тайпинг, новые сообщения).

## 6. Сервис Звонков (Call Service)
* `GetCallLink`: Получить ссылку для начала звонка (WebRTC/VAPI).

## 7. Сервис Памяти (Memory Service)
* `GetMemory`: Получить факты, которые бот знает о пользователе (Read-only).
* `GetKVMemory`: Получить пользовательские настройки (Key-Value).
* `UpdateKVMemory`: Сохранить/обновить настройки.

## 8. Системный Сервис (System Service)
* `GetConfig`: Получить системную конфигурацию (локали, валюты).
* `StreamExchangeRates`: Стрим курсов валют.
