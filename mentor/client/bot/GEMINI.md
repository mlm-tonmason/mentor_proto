# mentor/client/bot/

## BotService

Клиентский сервис управления ботами для **авторов** — пользователей с правом создания ботов (`max_bots > 0`).

### RPC-методы

| Метод | Описание |
|-------|----------|
| `CreateBot` | Создать бота (проверка лимита `current < max_bots`) |
| `UpdateBot` | Обновить данные (имя, описание, аватар, теги) |
| `DeleteBot` | Мягкое удаление → ARCHIVED |
| `GetBot` | Детали своего бота |
| `ListMyBots` | Список своих ботов с фильтром по статусу |
| `UpdateBotStatus` | Смена статуса: DRAFT→MODERATION, PUBLISHED→ARCHIVED |
| `CreatePricingPlan` | Создать тарифный план (DRAFT) |
| `UpdatePricingPlan` | Обновить тариф |
| `ListMyPricingPlans` | Список тарифов автора |
| `UpdatePricingPlanStatus` | Смена статуса плана: DRAFT→ACTIVE→ARCHIVED |
| `GetAuthorDashboard` | Сводка: лимиты, боты, подписчики |

### Ключевые модели

- **`CreateBotRequest`** — декомпозирован на отдельные поля (сервер контролирует `id`, `creator_id`, `status`)
- **`UpdateBotRequest`** — `optional` поля, обновляются только переданные
- **`AuthorDashboardResponse`** — `max_bots`, `current_bot_count`, `total_subscribers`, `bot_summaries`
- **`BotSummary`** — легковесная статистика по боту (подписчики, рейтинг, статус)

### Ограничения доступа

- Все операции доступны только **создателю** бота (проверка `creator_id`).
- Переход MODERATION → PUBLISHED — только через админ-сервис.
- Создание бота — проверка лимита `max_bots` (устанавливается через `AuthorAdminService`).
