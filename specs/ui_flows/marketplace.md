# Маркетплейс и Каталог (Marketplace)

## 1. Список Ботов - Двойники (Twins Tab)
Страница каталога с включенным фильтром по типу "Двойники" и активными фильтрами по категориям/свойствам.

![Twins List](./assets/marketplace_twins.png)

## 2. Список Ботов - Эксперты (Experts Tab)
Страница каталога с включенным фильтром по типу "Эксперты".

![Experts List](./assets/marketplace_experts.png)

### Блок: Фильтры (Sidebar / Top Bar)
* **API Метод**: `MarketService/SearchBots`.
* **Параметры**:
    *   `genre`: TWIN или EXPERT.
    *   `tags`: Список категорий.
    *   `sort_order`: CREATED_AT, RATING, PRICE.
* **Логика Фильтрации**:
    1.  **Категории (Tags)**: Множественный выбор.
    2.  **Свойства (Flags)**:
        *   **Hot** (`is_hot`).
        *   **Favorites** (локальный фильтр или отдельный запрос `ListMyLibrary`).

### Блок: Сетка Ботов (Bots Grid)
* **Модель**: `Bot` (Краткая информация).
* **Элементы Карточки**:
    *   **Аватар и Имя**.
    *   **Описание**: `short_description`.
    *   **Бейджи**: `HOT`, `EXPERT` (v2).
    *   **Рейтинг**: `rating.average` (★ 4.8).
    *   **Действия**:
        *   Клик -> Переход на страницу [Карточка Бота](./bot_details.md).
        *   Like/Star -> `MarketService/AddToFavorites` / `RemoveFromFavorites`.
