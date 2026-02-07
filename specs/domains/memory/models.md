# Домен Памяти (Memory Domain)

## Модели

### AvatarUserMemory (Память Аватара - Structured)
Персональная "память" (факты, профиль), заполняемая ботом на основе шаблона.
* **userId** (u32): Пользователь.
* **botId** (u32): Бот.
* **data** (json): Структурированные данные (например, `{"name": "Mike", "age": 30}`).
* **schemaVersionId** (u32): Ссылка на версию шаблона.

### AvatarUserKVMemory (Настройки / KV Storage)
Дополнительное хранилище "плоских" настроек для пары Пользователь-Бот.
* **userId** (u32): Пользователь.
* **botId** (u32): Бот.
* **data** (map<string, string>): Плоский JSON (ключ-значение).
* **constraints**:
    * Бэкенд автоматически иньецирует эти данные в System Prompt с префиксом.
    * Пример: ключ `theme` -> промпт `USER_THEME: dark`.
    * Управляется клиентом (Client API) или ботом.

### DialogueSummary (Саммари Диалога / Samurai’s)
Краткое содержание (выжимка) диалоговой сессии.
* **dialogueSessionId** (uuid): Ссылка на сессию.
* **content** (string): Текст саммари.

### UserProfileTemplate (Шаблон Профиля / Admin)
Шаблон профиля (структура вопросов/полей), который бот должен заполнить о пользователе.
* **id** (uuid): ID шаблона.
* **name** (string): Название (например, "CryptoExpertTemplate").
* **description** (string): Описание назначения шаблона.

### UserProfileTemplateVersion (Версия Шаблона)
Версия шаблона профиля (для поддержки обратной совместимости).
* **templateId** (uuid): Ссылка на шаблон.
* **version** (string/int): Номер версии.
* **schema** (json): JSON Schema, описывающая поля и вопросы.

### BotUserProfileBinding (Привязка)
Связь бота с определенным шаблоном профиля.
* **botId** (uuid): Бот.
* **templateId** (uuid): Шаблон.
