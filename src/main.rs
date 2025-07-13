mod plugin_udp;
mod plugin_obs;
mod plugin_store;
mod plugin_playback;
mod plugin_license;

use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("reStrike VTA backend starting...");
    
    // Start UDP server
    if let Err(e) = plugin_udp::start_udp_server().await {
        eprintln!("Failed to start UDP server: {}", e);
        std::process::exit(1);
    }
}
