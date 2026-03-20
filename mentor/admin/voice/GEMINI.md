# mentor/admin/voice/

## VoiceAdminService

Админ-сервис управления голосами — stub, ожидает интеграции VAPI.

### RPC-методы

| Метод | Описание |
|-------|----------|
| `ListAvailableVoices` | Список доступных голосов (по языку) |
| `RegisterVoice` | Регистрация нового голоса в системе |

### Модели

- **`Voice`** — `id`, `display_name`, `language_code`, `gender`, `style`, `provider_name`

### Статус

⏸️ **Stub** — все методы возвращают `Unimplemented`. Будет реализовано при интеграции с VAPI (провайдер голосовых AI-звонков).
