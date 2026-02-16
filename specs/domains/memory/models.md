# Домен Памяти (Memory Domain)

## Модели

### BotUserMemory (Долгосрочная память)
Персональная память бота о пользователе (факты, саммари).
* **distributor_id** (u32): Пользователь (Дистрибьютор).
* **bot_id** (u32): Бот.
* **data** (google.protobuf.Struct): Структурированные данные, извлеченные LLM из диалога.
    * *Пример*: `{ "name": "Mike", "hobbies": ["web3", "crypto"] }`.
* **updated_at** (timestamp): Время последнего обновления.

### BotUserKVMemory (Настройки / KV Storage)
Хранилище произвольных пар ключ-значение для настроек или состояния диалога.
* **distributor_id** (u32): Пользователь.
* **bot_id** (u32): Бот.
* **items** (map<string, string>): Данные (Key-Value).
    * *Пример*: `"theme": "dark", "last_topic": "NFT"`.
