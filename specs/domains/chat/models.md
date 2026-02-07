# Домен Чат (Chat Domain)

## Модели

### ChatThread (Поток Чата)
Поток чата (диалог) между пользователем и ботом.
* **id** (u32): Уникальный идентификатор чата.
* **userId** (u32): Ссылка на пользователя.
* **botId** (u32): Ссылка на бота.
* **isArchived** (boolean): Находится ли чат в архиве (по умолчанию false).
* **isPinned** (boolean): Закреплен ли чат в списке (макс 5 на юзера).
* **responseMode** (enum): TEXT, VOICE. Предпочтительный формат ответа бота.
* **settings** (object): Персональные настройки чата.
* **constraints**: Один пользователь может иметь множество диалогов с одним ботом (аналог ChatGPT).

### DialogueSession (Диалоговая Сессия)
Логическая сессия внутри ChatThread (используется для контекста/памяти).
* **id** (u32): Уникальный идентификатор сессии.
* **chatThreadId** (u32): Ссылка на ChatThread.
* **startedAt** (timestamp): Время начала сессии.
* **endedAt** (timestamp, optional): Время завершения сессии.
* **lastUserActivityAt** (timestamp): Время последней активности пользователя.
* **inactivityTimeoutSec** (int): Таймаут неактивности (по умолчанию 7200 сек = 2 часа).
* **channelType** (enum): Тип канала (chat / call / mixed).

### Message (Сообщение)
Сообщение в чате. Структура, объединяющая контент и метаданные.
* **id** (u32): Уникальный идентификатор сообщения.
* **chatThreadId** (u32): Ссылка на ChatThread.
* **dialogueSessionId** (u32): Ссылка на DialogueSession.
* **role** (enum): Роль отправителя (user / assistant / system).
* **createdAt** (timestamp): Время создания.
* **content**:
    * **text** (string, optional): Текстовое содержимое (или транскрипция для голоса).
    * **type** (enum): TEXT, AUDIO, IMAGE, VIDEO, FILE.
    * **metadata** (object): Мета-информация о вложении (не сам контент).
        * `mimeType` (string): Тип файла.
        * `sizeBytes` (int): Размер файла.
        * `fileName` (string, optional): Имя файла.
        * **AudioSpecific**:
            * `durationSeconds` (int): Продолжительность.
            * `transcription` (string): Текст расшифровки (дублируется в text или отдельно).
        * **Image/VideoSpecific**:
            * `width` (int): Ширина.
            * `height` (int): Высота.
            * `previewUrl` (string): Ссылка на превью (опционально).

### ChatState (Realtime)
Текущее состояние диалога (эфимерное событие).
* **chatThreadId** (u32): ID чата.
* **status** (enum):
    * `bot_typing`: Бот генерирует ответ.
    * `bot_recording`: Бот записывает голосовое (для аудио-ответов).

## Модели Realtime (Звонки)

### CallSession (Сессия звонка)
Сессия голосового звонка.
* **id** (u32): Уникальный идентификатор звонка.
* **chatThreadId** (u32): Ссылка на ChatThread.
* **dialogueSessionId** (uuid): Ссылка на DialogueSession (звонок является частью диалога).
* **providerCallId** (string): ID звонка на стороне Provider.
* **startedAt** (timestamp): Время начала.
* **endedAt** (timestamp): Время завершения.
* **voiceId** (string): Использованный голос.

### CallTranscript (Транскрипт)
Текстовая расшифровка звонка.
* **callSessionId** (uuid): Ссылка на CallSession.
* **text** (string): Полный текст расшифровки.
