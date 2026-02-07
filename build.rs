fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Компиляция proto файлов...");

    tonic_prost_build::configure()
        .out_dir("src/generated") // Директория для сгенерированного кода
        .build_server(true) // Генерируем серверную часть
        .build_client(false) // Клиентская часть не нужна (используем postman/frontend)
        .file_descriptor_set_path("src/generated/mentor_file_descriptor.bin") // Для gRPC reflection
        .compile_protos(
            &[
                // Типы данных (Types)
                "mentor/types/common.proto",
                "mentor/types/identity.proto",
                "mentor/types/billing.proto",
                "mentor/types/bot.proto",
                "mentor/types/chat.proto",
                "mentor/types/memory.proto",
                // Клиентские Сервисы (Client Services)
                "mentor/client/auth/auth.proto",
                "mentor/client/identity/identity.proto",
                "mentor/client/billing/billing.proto",
                "mentor/client/bot/bot.proto",
                "mentor/client/chat/chat.proto",
                "mentor/client/memory/memory.proto",
                "mentor/client/system/system.proto",
                // Административные Сервисы (Admin Services)
                // Пока не реализованы
            ],
            &["."], // Корневая директория для поиска импортов
        )
        .unwrap_or_else(|e| panic!("Ошибка компиляции proto файлов: {:?}", e));

    println!("Proto файлы успешно скомпилированы.");
    Ok(())
}
