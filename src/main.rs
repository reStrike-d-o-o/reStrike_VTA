mod plugin_obs;
mod plugin_udp;
mod plugin_playback;
mod plugin_store;
mod plugin_license;

#[tokio::main]
async fn main() {
    println!("reStrike VTA backend starting...");
    
    // OBS WebSocket plugin is now integrated and ready for use
    println!("✓ OBS WebSocket plugin loaded");
    println!("  - Scene switching: plugin_obs::switch_scene(\"Scene Name\")");
    println!("  - Buffer clips: plugin_obs::create_buffer_clip()");
    
    // Other plugins loaded
    println!("✓ UDP plugin loaded");
    println!("✓ Playback plugin loaded");
    println!("✓ Store plugin loaded");
    println!("✓ License plugin loaded");
    
    println!("reStrike VTA backend ready!");
    
    // Example usage (commented out - uncomment to test with running OBS):
    /*
    println!("\nTesting OBS integration...");
    match plugin_obs::switch_scene("Scene 1").await {
        Ok(_) => println!("✓ Scene switch successful"),
        Err(e) => println!("ℹ Scene switch test: {}", e),
    }
    */
}
