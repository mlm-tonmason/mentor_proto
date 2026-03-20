# mentor/client/billing/

## BillingService

Сервис биллинга — покупка подписок, управление подписками, финансы.

### RPC-методы

| Метод | Описание |
|-------|----------|
| `GetSubscriptionDashboard` | Агрегированный дашборд (подписки, тарифы, боты, авторы) |
| `PurchaseBotSubscription` | Покупка/смена тарифного плана |
| `GetMySubscriptions` | Список всех подписок с фильтром по статусу |
| `GetSubscriptionQuotas` | Детальный расход квот (Available/Used/Reserved/Total) |
| `CancelSubscription` | Отмена подписки (дорабатывает до конца оплаченного срока) |
| `ToggleAutoRenew` | Вкл/выкл автопродления |
| `GetBalances` | Балансы по всем валютам |
| `GetTransaction` | Детали одной транзакции |
| `GetTransactions` | История транзакций с курсорной пагинацией |

### Ключевые модели

- **`QuotaUsage`** — расход квоты: `total`, `used`, `reserved`, `available`, привязанные боты
- **`GetMySubscriptionsResponse`** — подписки + связанные тарифы + боты одним запросом
- **`CancelSubscriptionResponse`** — подписка переходит в CANCELLED, но продолжает действовать

### Особенности

- Покупка через `PurchaseBotSubscription` инициирует транзакцию во внешнем сервисе (Network).
- Пользователь не взаимодействует с платёжной системой напрямую.
