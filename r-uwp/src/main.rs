// Dependencies
use clap::Parser;
mod uwp;
mod ws;
mod commands;

/// General and exploit fixes for UWP Roblox.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Disable the websocket server.
    #[arg(short='w', long, default_value = "false")]
    pub disable_ws: bool,

    /// The port the websocket server is attached to.
    #[arg(short, long, default_value = "8080")]
    pub port: u16,

    /// The shift lock key to use. Provide the key id.
    #[arg(short='k', long, default_value_t = 0xA0)]
    pub shift_lock_key: u64,

    /// Disables clipping the mouse to the window during right click.
    #[arg(short='c', long, default_value = "false")]
    pub disable_clip_mouse: bool,

    /// Disables clipping the mouse to the window during shift lock.
    #[arg(short='l', long, default_value = "false")]
    pub disable_clip_shift: bool,

    /// Disables the mouse teleport fix.
    #[arg(short='m', long, default_value = "false")]
    pub disable_mouse_tp: bool,

    /// Disables the teleport crash fix.
    #[arg(short='t', long, default_value = "false")]
    pub disable_tp_crash: bool,

    /// Supresses all output messages.
    #[arg(short, long, default_value = "false")]
    pub silent: bool,
}

/// Entrypoint.
#[actix_web::main]
async fn main() {
    // Parse the commandline arguments
    let args = Args::parse();
    let port = if !args.disable_ws {
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

    // So the exploit can connect to localhost
    if let Err(e) = std::process::Command::new("cmd")
        .args(&["/C", "CheckNetIsolation", "LoopbackExempt", "-a", "-n=\"ROBLOXCORPORATION.ROBLOX_55nm5eh3cm0pr\""])
        .output() {
            log::warn!("Failed to install the loopback exemption: {}. Please execute the `CheckNetIsolation` command manually for exploit fixes, view GitHub readme for more information.", e);
        };
    log::info!("Successfully installed the loopback exemption");

    // Run the WSS
    if let Some(port) = port {
        ws::start(port)
            .await
            .expect("failed to start websocket server");
    }
}