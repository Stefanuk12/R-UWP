// Dependencies
use clap::Parser;
mod uwp;
mod ws;
mod commands;

/// General and exploti fixes for UWP Roblox.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Enable the websocket server.
    #[arg(short, long, default_value = "true")]
    pub ws: bool,

    /// The port the websocket server is attached to.
    #[arg(short, long, default_value = "8080")]
    pub port: u16,

    /// Clips the mouse to the window during right click.
    #[arg(short, long, default_value = "true")]
    pub clip_mouse: bool,

    /// Attempts to teleport the mouse back after letting go of right click.
    #[arg(short, long, default_value = "true")]
    pub mouse_tp: bool,

    /// Attempts to fix teleport crashes.
    #[arg(short, long, default_value = "true")]
    pub tp_crash: bool,

    /// Supresses all messages.
    #[arg(short, long, default_value = "false")]
    pub silent: bool,
}

/// Entrypoint.
#[actix_web::main]
async fn main() {
    // Parse the commandline arguments
    let args = Args::parse();
    let port = if args.ws {
        Some(args.port.clone())
    } else {
        None
    };

    // Start `env_logger`
    if args.silent {
        std::env::set_var("RUST_LOG", "none");
    }
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Start the UWP fixes
    std::thread::spawn(move || {
        uwp::start_uwp(args);
    });

    // Run the WSS
    if let Some(port) = port {
        ws::start(port)
            .await
            .expect("failed to start websocket server");
    }
}