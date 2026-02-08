# Домен Бот (Bot Domain)

## Модели

### Bot (Бот / Эксперт / Двойник)
Сущность бота-эксперта или двойника, отображаемая в Маркетплейсе.

* **id** (u32): Уникальный идентификатор.
* **system_name** (string): Уникальный системный идентификатор (handle), например `@elon_musk_twin`.
* **display_name** (string): Отображаемое имя (например, "Elon Musk").
* **avatar_url** (string): URL изображения обложки/бота.
* **short_description** (string): Краткое описание для карточки.
* **long_description** (string): Полное описание для детальной страницы.

* **creator** (PublicUserProfile): Инфо о создателе бота.

* **genre** (enum Genre.Id):
    * `UNSPECIFIED`: Не указан.
    * `TWIN`: Двойник (создан пользователем).
    * `EXPERT`: Эксперт (создан компанией).

* **status** (enum Status.Id):
    * `DRAFT`: Черновик.
    * `MODERATION`: На модерации.
    * `PUBLISHED`: Опубликован.
    * `ARCHIVED`: В архиве.
    * `BLOCKED`: Заблокирован.

* **tags** (list string): Теги категорий.

* **Flags**:
    * `is_hot` (bool): Трендовый.
    * `is_archived` (bool): В архиве (контекстно).
    * `is_favorite` (bool): В избранном (контекстно).
    * `is_subscribed` (bool): Есть активная подписка (контекстно).

* **rating** (mentor.types.Rating): Статистика рейтинга (средняя оценка, распределение).

* **version** (u32): Версия бота.
* **updated_at** (timestamp): Дата последнего обновления.
