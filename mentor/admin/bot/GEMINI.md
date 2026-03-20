# mentor/admin/bot/

## BotAdminService

Админ-сервис управления ботами — полный CRUD, статусы, тренды, тарифы, квоты.

### RPC-методы

| Группа | Метод | Описание |
|--------|-------|----------|
| CRUD | `CreateBot` | Создание бота |
| | `UpdateBot` | Обновление полей |
| | `DeleteBot` | Мягкое удаление |
| | `ListBots` | Все боты с пагинацией |
| Статусы | `UpdateBotStatus` | Смена статуса (Draft → Published → Archived) |
| | `SetBotHot` | Пометить как трендовый (только админ) |
| Тарифы | `CreatePricingPlan` | Создать тарифный план |
| | `UpdatePricingPlan` | Обновить тариф |
| Квоты | `CreateQuota` | Создать именованную квоту в плане |
| | `BindBotToQuota` | Привязать бота к квоте |
| | `UnbindBotFromQuota` | Отвязать бота от квоты |
| | `ListPlanQuotas` | Все квоты плана с привязками |

### Ключевые модели

- **`UsageType`** — enum: `CHAT_MESSAGE`, `VOICE_CALL`
- **`QuotaWithBindings`** — квота + список привязанных ботов
- **`BotBinding`** — привязка бота к квоте с типом использования
