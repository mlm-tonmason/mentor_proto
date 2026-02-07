fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Compiling proto files...");

    tonic_prost_build::configure()
        .out_dir("src/generated")
        .build_server(true)
        .build_client(false)
        .file_descriptor_set_path("src/generated/mentor_file_descriptor.bin")
        .compile_protos(
            &[
                // Types
                "mentor/types/common.proto",
                "mentor/types/identity.proto",
                "mentor/types/billing.proto",
                "mentor/types/bot.proto",
                "mentor/types/chat.proto",
                "mentor/types/memory.proto",
                // Client Services
                "mentor/client/auth/auth_service.proto",
                "mentor/client/identity/identity_service.proto",
                "mentor/client/billing/billing_service.proto",
                "mentor/client/bot/bot_service.proto",
                "mentor/client/chat/chat_service.proto",
                "mentor/client/memory/memory_service.proto",
                // Admin Services (Not implemented yet, removing placeholders)
            ],
            &["."],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos: {:?}", e));

    println!("Proto files compiled successfully.");
    Ok(())
}
