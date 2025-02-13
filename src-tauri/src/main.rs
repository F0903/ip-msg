// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::panic::set_hook(Box::new(|panic_info| {
        log::error!("fatal panic occurred: {}", panic_info);
    }));

    match ip_msg_lib::run().await {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("error occurred: {}", e);
            Err(e)
        }
    }
}
