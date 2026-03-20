# mentor/client/memory/

## MemoryService

Сервис управления памятью бота о пользователе (RAG-контекст).

### RPC-методы

| Метод | Описание |
|-------|----------|
| `GetMemory` | Получить факты, которые бот знает о пользователе (заполняются LLM) |
| `GetKVMemory` | Получить настройки «Ключ-Значение» |
| `UpdateKVMemory` | Сохранить произвольные пары ключ-значение |

### Два типа памяти

1. **BotUserMemory** — структурированные факты, извлечённые LLM из диалогов (`google.protobuf.Struct`)
2. **BotUserKVMemory** — плоское хранилище `map<string, string>`, инжектируется в System Prompt (`USER_THEME=Dark`)
