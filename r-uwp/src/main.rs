// Dependencies
mod uwp;
mod ws;
mod commands;

/// Entrypoint.
#[actix_web::main]
async fn main() {
    // Start `env_logger`
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Start the UWP fixes
    std::thread::spawn(uwp::start_uwp);

    // Run the WSS
    let _ = ws::start().await;
}