# mentor/client/chat/

## ChatService

Сервис чата — диалоги с ботами, сообщения, контакты, реалтайм-события.

### RPC-методы

| Группа | Метод | Описание |
|--------|-------|----------|
| Контакты | `ListContactBots` | Список ботов с активными диалогами (левое меню) |
| | `PinBot` / `UnpinBot` | Закрепить/открепить бота |
| | `ReorderPinnedBots` | Изменить порядок закреплённых |
| Диалоги | `CreateChat` | Создать новый диалог с ботом |
| | `ListChats` | Список диалогов с конкретным ботом |
| | `UpdateChatTitle` | Переименовать диалог |
| | `UpdateChatConfiguration` | Изменить режим ответа (текст/голос), thinking mode |
| | `DeleteChat` | Удалить диалог |
| Сообщения | `GetHistory` | История с пагинацией и фильтром по избранным |
| | `SendMessage` | Отправить текст или аудио |
| | `AddToFavorites` / `RemoveFromFavorites` | Избранные сообщения |
| | `DownloadAudio` | Скачать аудиофайл |
| Real-time | `SubscribeToEvents` | Server-stream: typing, new_message, chat lifecycle, pin changes |

### Особенности

- Один бот может иметь **несколько диалогов** с одним пользователем.
- `ChatThread.Configuration` — режим ответа (TEXT/VOICE) и Sinking Mode.
- `SubscribeToEvents` — глобальный стрим для всех ботов/чатов пользователя.
