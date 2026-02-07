# Профиль и Личные Данные

### Блок: Вход через Telegram (Auth Widget)

* **Сценарий**:
    *   Пользователь открывает приложение.
    *   Если нет активного JWT токена -> Редирект на экран входа.
* **Элементы**:
    *   Логотип Mentor / TonMason.
    *   **Telegram Login Widget** (Официальный скрипт `<script async src="https://telegram.org/js/telegram-widget.js?...">`).
    *   Кнопка "Log in with Telegram".
* **Логика**:
    1.  Пользователь нажимает "Log in".
    2.  Telegram возвращает `user_id`, `hash`, `auth_date`.
    3.  Фронтенд отправляет эти данные на `AuthService/Login(TelegramAuthRequest)`.
    4.  Бэкенд валидирует хэш, ищет/создает пользователя (Lazy Registration).
    5.  Возвращает `JWT` токен.
    6.  Фронтенд сохраняет JWT и переходит в `My Library`.
Страница для управления сессией и локальными настройками.

![Profile Screen](./assets/profile_settings.png)

### Блок: Личные Данные (Header)
* **API Метод**: `IdentityService/GetMyProfile`
* **Модель**: `UserProfile` + `User.networkData`
* **Отображение**:
    * **Аватар**: `networkData.avatarUrl` (Круглый, 64x64).
    * **Username**: `networkData.username`.
    * **Статус**: "Logged in via Telegram".
* **Действия**:
    * **Выйти**: Очистка `JWT` и редирект на Login. (Метод `AuthService/Logout` опционален, достаточно удалить токен, так как он stateless).

### Блок: Настройки (Settings)
* **Логика**:
    * Настройки `Theme` и `Animations` хранятся в **LocalStorage** браузера и не требуют запросов к бэкенду.
    * Настройка `Language` синхронизируется, чтобы переводы приходили корректные.
* **Поля**:
    1.  **Language (Язык)**:
        *   Выпадающий список.
        *   При изменении: `IdentityService/UpdateSettings({ interfaceLanguage: "ru" })`.
    2.  **Theme (Тема)**:
        *   Переключатель Sun/Moon.
        *   **Local Only**: Применяет CSS класс `dark-theme` / `light-theme`.
    3.  **Animations (Анимации)**:
        *   Тоггл "Включить/выключить".
        *   **Local Only**: Отключает тяжелые JS/CSS эффекты.
