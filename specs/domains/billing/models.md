# Домен Биллинга (Billing Domain)

## Модели

### BotTariffPlan (Тарифный План Бота)
Публичная модель тарифного плана для отображения в каталоге.
* **plan_id** (u32): Уникальный идентификатор плана.
* **creator_id** (u32): Создатель (дистрибьютор), владеющий планом.
* **status** (enum):
    * `DRAFT`: Черновик.
    * `ACTIVE`: Активен.
    * `ARCHIVED`: В архиве.
* **month_price** (Money): Стоимость за месяц (в базовой валюте).
* **duration_months** (u32): Длительность контракта в месяцах.
* **quota_metrics** (map<u32, QuotaMetrics>): Лимиты ресурсов (сообщения, звонки) для каждой квоты.
* **bot_quota** (map<u32, u32>): Привязка бота к ID квоты.

### BotSubscription (Подписка на Бота)
Экземпляр активной подписки пользователя.
* **id** (u32): Внутренний ID подписки.
* **nonce** (u32): Порядковый номер покупки у данного автора.
* **creator_id** (u32): На кого подписан.
* **distributor_id** (u32): Кто купил (пользователь).
* **plan_id** (u32): Ссылка на тариф.
* **status** (enum):
    * `ACTIVE`: Активна.
    * `CANCELLED`: Отменена (дорабатывает срок).
    * `EXPIRED`: Истекла.
    * `SUSPENDED`: Приостановлена.
* **period_quota_metrics_used** (map<u32, PeriodQuotaMetricsUsed>): Использованные ресурсы в текущем периоде.
* **total_period** (DateRange): Общий срок действия.
* **next_renewal_at** (timestamp, optional): Дата автопродления.

### SubscriptionDashboardData
Агрегированные данные для экрана "Мои подписки".
* **creators** (list PublicUserProfile): Авторы, на которых есть подписка.
* **bot_tariff_plans** (list BotTariffPlan): Актуальные тарифы.
* **bot_subscriptions** (list BotSubscription): Сами подписки.
* **bots** (list Bot): Боты, доступные по подпискам.

### Transaction (Транзакция)
Финансовая операция.
* **id** (u32): ID транзакции.
* **distributor_id** (u32): Плательщик.
* **status** (enum): `PENDING`, `COMPLETED`, `FAILED`.
* **amount_base** (Money): Сумма в базовой валюте (для отчетов).
* **amount_charged** (Money): Сумма списания (в крипте/фиате).
* **created_at** (timestamp): Дата создания.
* **metadata** (oneof):
    * `bot_subscription`: Детали покупки подписки (ID подписки, период).
