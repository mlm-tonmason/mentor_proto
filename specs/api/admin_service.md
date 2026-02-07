# Спецификация Админ API

## 1. Админ-сервис Ботов (Bot Admin Service)
* `CreateBot`: Создание базовой сущности (ID, Name, Description, AvatarUrl).
* `UpdateBot` / `DeleteBot`: CRUD операции.
* `ListBots`: Список всех ботов.
* `CreatePricingPlan` / `UpdatePricingPlan`: Управление тарифами.

## 2. Админ-сервис Профилей (Profile Admin Service)
* `CreateTemplate` / `UpdateTemplate`: Управление шаблонами профилей.
* `CreateTemplateVersion`: Версионирование шаблонов.
* `BindBotToTemplate`: Привязка шаблона к боту.

## 3. Админ-сервис Голосов (Voice Admin Service)
* `ListAvailableVoices`: Список доступных голосов.
* `RegisterVoice`: Регистрация нового голоса (провайдера).
