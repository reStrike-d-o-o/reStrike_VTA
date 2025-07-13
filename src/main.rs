mod plugin_license;
mod plugin_obs;
mod plugin_playback;
mod plugin_store;
mod plugin_udp;

fn main() {
    println!("reStrike VTA backend starting...");

    // Initialize plugins
    plugin_license::check_license();
    plugin_obs::connect_obs();
    plugin_playback::playback_clip();
    plugin_store::store_data();
    plugin_udp::start_udp_server();
}
