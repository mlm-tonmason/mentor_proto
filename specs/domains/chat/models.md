# Домен Чат (Chat Domain)

## Модели

### ChatThread (Диалог)
Поток сообщений между пользователем и ботом.
* **id** (u32): Уникальный ID чата.
* **bot_id** (u32): Ссылка на бота.
* **distributor_id** (u32): Владелец чата (пользователь).
* **title** (string): Название чата (изменяемое).
* **configuration** (Configuration):
    * `response_mode` (enum): TEXT, VOICE.
    * `sinking_mode_enabled` (bool): "Thinking mode" (длительная генерация).
* **last_message** (Message): Последнее сообщение (для превью).

### Message (Сообщение)
Единица обмена информацией в чате.
* **id** (u64): Уникальный ID сообщения (Snowflake).
* **thread_id** (u32): Ссылка на чат.
* **role** (enum Role.Id): `USER`, `BOT`, `SYSTEM`.
* **created_at** (timestamp): Время отправки.
* **is_favorite** (bool): Добавлено ли в избранное.
* **content** (oneof):
    * `text` (string): Текст (Markdown).
    * `audio` (AudioContent):
        * `url` (string).
        * `transcription` (string).
        * `duration_seconds` (i32).

### ChatEvent (Real-time событие)
Единый стрим для всех чат-событий: реалтайм + жизненный цикл.
Приходит в стрим `SubscribeToEvents`.

**oneof event:**
* **typing** (TypingIndicator): Индикатор набора текста/записи голоса.
    * `thread_id` (u32): ID чата.
    * `activity` (enum): `TYPING`, `RECORDING_VOICE`.
* **new_message** (Message): Новое пришедшее сообщение.
* **error** (Error): Ошибка стрима.
* **chat_created** (ChatThread): Новый диалог создан.
* **chat_updated** (ChatThread): Диалог обновлен (название, настройки, архивация).
* **chat_deleted** (ChatThread.Id): Диалог удален.
* **pin_changed** (PinChanged): Изменен состав/порядок закрепленных ботов.
    * `pinned_bot_ids` (list u32): Актуальный порядок.

