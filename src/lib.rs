/// Корневой модуль gRPC определений проекта Mentor.
///
/// Структура повторяет proto-пакеты:
/// - `mentor::types` — общие модели данных (используются во всех сервисах).
/// - `mentor::client` — клиентское API (Mobile / Web / Mini App).
pub mod mentor {
    /// Бинарный дескриптор всех proto-файлов для gRPC Server Reflection.
    pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("generated/mentor_file_descriptor.bin");

    /// Общие типы данных, используемые во всех слоях архитектуры.
    /// Базовые примитивы: Money, DateRange, Cursor, Rating.
    pub mod types {
        include!("generated/mentor.types.rs");

        /// Пользователи, публичные профили, локали, валюты, системная конфигурация.
        pub mod identity {
            include!("generated/mentor.types.identity.rs");
        }
        /// Модели ботов (Эксперты, Двойники), жанры, статусы.
        pub mod bot {
            include!("generated/mentor.types.bot.rs");
        }
        /// Чаты (ChatThread), сообщения (Message), real-time события (ChatEvent).
        pub mod chat {
            include!("generated/mentor.types.chat.rs");
        }
        /// Тарифные планы, подписки, транзакции, квоты.
        pub mod billing {
            include!("generated/mentor.types.billing.rs");
        }
        /// Память бота о пользователе (факты, KV-настройки).
        pub mod memory {
            include!("generated/mentor.types.memory.rs");
        }
    }

    /// Клиентское API для конечных пользователей.
    pub mod client {
        /// Аутентификация: Login (Telegram Mini App / Widget), Logout.
        pub mod auth {
            include!("generated/mentor.client.auth.rs");
        }
        /// Профиль текущего пользователя, смена языка.
        pub mod identity {
            include!("generated/mentor.client.identity.rs");
        }
        /// Каталог ботов: поиск, рейтинг, избранное, библиотека.
        pub mod market {
            include!("generated/mentor.client.market.rs");
        }
        /// Подписки, покупка тарифов, история транзакций.
        pub mod billing {
            include!("generated/mentor.client.billing.rs");
        }
        /// Диалоги, сообщения, контакты, real-time стрим событий.
        pub mod chat {
            include!("generated/mentor.client.chat.rs");
        }
        /// Голосовые звонки (WebRTC / VAPI).
        pub mod call {
            include!("generated/mentor.client.call.rs");
        }
        /// Память бота: факты о пользователе, KV-настройки.
        pub mod memory {
            include!("generated/mentor.client.memory.rs");
        }
        /// Системная конфигурация, стрим курсов валют.
        pub mod system {
            include!("generated/mentor.client.system.rs");
        }
    }
}
