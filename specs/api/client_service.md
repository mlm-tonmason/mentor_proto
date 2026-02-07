# Спецификация Клиентского API

Определения RPC, соответствующие высокоуровневым требованиям.

## 1. Сервис Аккаунта (Account Service)
* `GetProfile`: Получить профиль.
* `UpdateSettings`: Обновить настройки.

## 2. Сервис Маркета (Market Service)
* `ListBots`: Каталог ботов.
* `GetBot`: Получить одного бота.
* `AddToFavorites` / `RemoveFromFavorites`: Управление избранным.
* `ListMyLibrary`: Моя библиотека (подписки).
* `Subscribe`: Купить подписку.
* `GetQuotaDashboard`: Дашборд лимитов и квот.

## 3. Сервис Чата (Chat Service)
* `SendMessage`: Отправить сообщение.
* `GetHistory`: Получить историю чата.
* `GetBotChats`: Сайдбар (список диалогов с ботом).
* `PinChat` / `UnpinChat`: Закрепление диалогов.
* `ArchiveChat` / `UnarchiveChat`: Архивация.
* `ClearHistory`: Очистка истории.

## 4. Сервис Звонков (Call Service)
* `StartCall`: Начать звонок.
* `EndCall`: Завершить звонок.

## 5. Сервис Памяти (Memory Service)
* `GetAvatarMemory`: Получить память аватара о пользователе.
* `UpdateAvatarMemory`: Обновить память вручную.
* `ResetAvatarMemory`: "Забыть меня".

## 6. Системный Сервис (System Service)
* `GetConfig`: Получить конфиг (локали и т.д.).
* `StreamExchangeRates`: Стрим курсов валют.
