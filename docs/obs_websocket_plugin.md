# OBS WebSocket Plugin Documentation

## Overview

The OBS WebSocket plugin provides integration with OBS Studio through its WebSocket API, enabling scene switching and buffer clip functionality for the reStrike VTA application.

## Features

- **Scene Switching**: Switch between OBS scenes programmatically
- **Buffer Clip Creation**: Create instant replay clips using OBS replay buffer
- **Connection Management**: Robust connection handling with error management
- **Async API**: Full async/await support for non-blocking operations

## Quick Start

### Basic Usage

```rust
use restrike_vta::plugin_obs;

// Switch to a scene (connects, switches, and disconnects automatically)
plugin_obs::switch_scene("Scene 1").await?;

// Create a buffer clip (connects, saves replay buffer, and disconnects)
plugin_obs::create_buffer_clip().await?;
```

### Advanced Usage with Controller

```rust
use restrike_vta::plugin_obs::ObsController;

let mut controller = ObsController::new();

// Connect to OBS
controller.connect("localhost", 4455, None).await?;

// Switch scenes
controller.switch_scene("Scene 1").await?;
controller.switch_scene("Scene 2").await?;

// Work with replay buffer
controller.start_replay_buffer().await?;
// ... some time later ...
controller.save_replay_buffer().await?;

// Clean up
controller.disconnect().await;
```

## Configuration

### Default Settings
- **Host**: localhost
- **Port**: 4455 (OBS WebSocket default)
- **Password**: None

### Custom Configuration

```rust
use restrike_vta::plugin_obs;

// Connect with custom settings
plugin_obs::connect_and_switch_scene_with_config(
    "192.168.1.100",
    4455,
    Some("your_password"),
    "Scene 1"
).await?;
```

## API Reference

### Core Functions

- `switch_scene(scene_name: &str)` - Switch to a specific scene
- `create_buffer_clip()` - Create a clip from replay buffer
- `connect_and_switch_scene(scene_name: &str)` - Connect, switch scene, disconnect
- `connect_and_create_buffer_clip()` - Connect, create clip, disconnect

### ObsController Methods

- `new()` - Create new controller instance
- `connect(host, port, password)` - Connect to OBS WebSocket
- `disconnect()` - Disconnect from OBS WebSocket
- `switch_scene(scene_name)` - Switch to scene
- `get_current_scene()` - Get current scene name
- `list_scenes()` - Get list of all scenes
- `start_recording()` - Start recording
- `stop_recording()` - Stop recording
- `start_replay_buffer()` - Start replay buffer
- `stop_replay_buffer()` - Stop replay buffer
- `save_replay_buffer()` - Save replay buffer as clip
- `is_connected()` - Check connection status

## Error Handling

All functions return `Result<T, Box<dyn std::error::Error>>` for robust error handling:

```rust
match plugin_obs::switch_scene("Scene 1").await {
    Ok(_) => println!("Scene switched successfully"),
    Err(e) => eprintln!("Failed to switch scene: {}", e),
}
```

## OBS Studio Setup

1. Install OBS Studio
2. Install obs-websocket plugin (included in OBS 28+)
3. Enable WebSocket server in OBS:
   - Tools → obs-websocket Settings
   - Enable WebSocket server
   - Set port (default 4455)
   - Set password (optional but recommended)

## Integration with VTA Protocol

The OBS plugin integrates seamlessly with the VTA UDP protocol for automatic scene switching based on match events:

- Scene switching on match start/end
- Instant replay clips on significant points
- Automatic recording management

## Examples

See the `examples/` directory for complete usage examples:
- `basic_scene_switch.rs` - Simple scene switching
- `replay_buffer_demo.rs` - Replay buffer management
- `match_integration.rs` - Integration with VTA match events