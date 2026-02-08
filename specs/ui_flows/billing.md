# Управление Подписками

## 1. Страница Подписок (My Subscriptions)
Здесь пользователь видит свои подписки и доступные тарифы.

![Subscriptions](./assets/subscription_list.png)

### Блок: Список Ботов и Тарифов
* **API Метод**: `BillingService/GetSubscriptionDashboard`.
* **Модель**: `SubscriptionDashboardData`.
* **Логика Отображения**:
    1.  **Группировка**: Список ботов, на которых пользователь подписан.
    2.  **Активный Тариф**: Отображается текущая `BotSubscription`.
    3.  **Детали Тарифа**:
        *   **Цена**: `BotTariffPlan.month_price`.
        *   **Лимиты**: `quota_metrics` (остаток/лимит).
        *   **Даты**: `total_period` (start/end).
    4.  **Статус**:
        *   `ACTIVE`: Зеленый.
        *   `SUSPENDED`: Приостановлена.
        *   `EXPIRED`: Истекла.

### Действия (Management)
*На данный момент API управления подписками (Upgrade/Downgrade/Cancel) находится в разработке или реализуется через WebApp платежного провайдера.*

1.  **Продлить / Купить**:
    *   Кнопка "Buy Subscription".
    *   Инициирует создание `Transaction` (внешний флоу оплаты).
2.  **История операций**:
    *   Вкладка "Transactions".
    *   **API**: `BillingService/GetTransactions`.
