# Домен Бот (Bot Domain)

## Модели

### Bot (Бот / Эксперт / Двойник)
Сущность бота-эксперта или двойника, отображаемая в Маркетплейсе.
* **id** (u32): Уникальный идентификатор.
* **providerBotId** (string): ID у Provider (внешний сервис).
* **systemName** (string): Уникальный системный идентификатор (handle), например `@elon_musk_twin`.
* **displayName** (string): Отображаемое имя (например, "Elon Musk").
* **descriptions**:
    * `short` (string): Краткое описание для карточки (max 100 символов).
    * `long` (text): Полное описание для детальной страницы.
* **status** (enum):
    * `DRAFT`: Черновик.
    * `MODERATION`: На проверке.
    * `BLOCKED`: Заблокирован.
    * `PUBLISHED`: Доступен в каталоге.
    * `ARCHIVED`: Удален автором.
* **avatarUrl** (string): URL изображения обложки/бота.
* **ver**:
    * `updatedAt` (timestamp): Дата последнего изменения промпта/настроек. (Авто-миграция контекста, стратегия 4.2C).
    * `version` (u32): Инкрементальная версия конфига.
* **creator**:
    * `id` (u32): ID создателя.
    * `type` (enum): `COMPANY` (платформа) / `USER` (пользователь).
* **classification**:
    * `genre` (enum): `EXPERT` (если creator=COMPANY), `TWIN` (если creator=USER). Автоматически определяется.
    * `tags` (list string): Теги категорий (Бизнес, Здоровье, IT).
* **stats**:
    * `isExpert` (boolean): "Included TonMason" (эксперт компании).
    * `isHot` (boolean): Популярный/Трендовый.
    * `isFavorite` (boolean): В избранном у текущего пользователя.
    * `rating`:
        * `average` (f32): Средняя оценка (1.0 - 5.0).
        * `count` (u32): Общее количество оценок.
        * `distribution` (map<u32, u32>): Распределение (ключ - балл, значение - кол-во).
            * *Пример*: `5: 120, 4: 20, 1: 2`. Позволяет рисовать гистограмму.
    * `subscribersCount` (u32): Количество активных подписчиков.
* **voiceOptions** (object): Настройки голоса.
    * `allowedVoiceIds` (list string): Список допустимых ID голосов (от Provider).
    * `defaultVoiceId` (string): Голос по умолчанию.

### BotRating (Оценка Бота)
Персональная оценка пользователя боту.
* **botId** (u32): Ссылка на бота.
* **userId** (u32): Ссылка на пользователя.
* **score** (u32): Оценка (1-5 звезд).
* **createdAt** (timestamp): Дата оценки.
* **constraints**: Один пользователь - Одна оценка на бота (можно обновлять).
