# mentor/types/

Общие доменные модели, переиспользуемые клиентскими и административными сервисами.

## Файлы

| Файл | Пакет | Описание |
|------|-------|----------|
| `common.proto` | `mentor.types` | Базовые типы: `Money`, `DateRange`, `Cursor` (пагинация), `Rating` |
| `identity.proto` | `mentor.types.identity` | Профили пользователей (`UserProfile`, `PublicUserProfile`), локали, валюты, курсы, `SystemConfig` |
| `bot.proto` | `mentor.types.bot` | Сущность `Bot` — эксперт/двойник с жанром, статусом, рейтингом, тегами |
| `billing.proto` | `mentor.types.billing` | Биллинг: `BotTariffPlan`, `BotSubscription`, `Transaction`, `SubscriptionDashboardData` |
| `chat.proto` | `mentor.types.chat` | Чат: `ChatThread` (диалог), `Message` (сообщение), `ChatEvent` (реалтайм-события) |
| `memory.proto` | `mentor.types.memory` | Память бота о пользователе: `BotUserMemory` (факты от LLM), `BotUserKVMemory` (ключ-значение) |

## Ключевые паттерны

- **Вложенные идентификаторы**: каждая сущность имеет `message Id { ... }` и `message List { ... }`.
- **Обёрнутые enum-ы**: `message Status { enum Id { ... } }` — для расширяемости.
- **Money**: строковый формат суммы (`"10.50"`) для точного представления decimal.
- **Cursor**: курсорная пагинация по ID с направлением сортировки.
