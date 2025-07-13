// OBS controller plugin with scene switching and buffer clip functionality
use obws::Client;
use std::time::Duration;
use tokio::time::sleep;

/// OBS WebSocket client wrapper for scene control and clip commands
pub struct ObsController {
    client: Option<Client>,
    connected: bool,
}

impl ObsController {
    /// Create a new OBS controller instance
    pub fn new() -> Self {
        Self {
            client: None,
            connected: false,
        }
    }

    /// Connect to OBS WebSocket server
    pub async fn connect(&mut self, host: &str, port: u16, password: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        println!("Connecting to OBS WebSocket at {}:{}", host, port);
        
        let client = match password {
            Some(pwd) => Client::connect(host, port, Some(pwd)).await?,
            None => Client::connect(host, port, Option::<&str>::None).await?,
        };
        
        self.client = Some(client);
        self.connected = true;
        println!("Successfully connected to OBS WebSocket");
        
        Ok(())
    }

    /// Disconnect from OBS WebSocket server
    pub async fn disconnect(&mut self) {
        if let Some(mut client) = self.client.take() {
            client.disconnect().await;
            self.client = None;
            self.connected = false;
            println!("Disconnected from OBS WebSocket");
        }
    }

    /// Switch to a specific scene
    pub async fn switch_scene(&self, scene_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !self.connected {
            return Err("Not connected to OBS WebSocket".into());
        }

        if let Some(client) = &self.client {
            println!("Switching to scene: {}", scene_name);
            client.scenes().set_current_program_scene(scene_name).await?;
            println!("Successfully switched to scene: {}", scene_name);
        }
        
        Ok(())
    }

    /// Get current scene name
    pub async fn get_current_scene(&self) -> Result<String, Box<dyn std::error::Error>> {
        if !self.connected {
            return Err("Not connected to OBS WebSocket".into());
        }

        if let Some(client) = &self.client {
            let scene = client.scenes().current_program_scene().await?;
            return Ok(format!("{:?}", scene.id));
        }
        
        Err("Client not available".into())
    }

    /// Start recording (buffer clip functionality)
    pub async fn start_recording(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.connected {
            return Err("Not connected to OBS WebSocket".into());
        }

        if let Some(client) = &self.client {
            println!("Starting recording for buffer clip");
            client.recording().start().await?;
            println!("Recording started successfully");
        }
        
        Ok(())
    }

    /// Stop recording (buffer clip functionality)
    pub async fn stop_recording(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.connected {
            return Err("Not connected to OBS WebSocket".into());
        }

        if let Some(client) = &self.client {
            println!("Stopping recording for buffer clip");
            client.recording().stop().await?;
            println!("Recording stopped successfully");
        }
        
        Ok(())
    }

    /// Start replay buffer (instant replay clip functionality)
    pub async fn start_replay_buffer(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.connected {
            return Err("Not connected to OBS WebSocket".into());
        }

        if let Some(client) = &self.client {
            println!("Starting replay buffer");
            client.replay_buffer().start().await?;
            println!("Replay buffer started successfully");
        }
        
        Ok(())
    }

    /// Stop replay buffer
    pub async fn stop_replay_buffer(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.connected {
            return Err("Not connected to OBS WebSocket".into());
        }

        if let Some(client) = &self.client {
            println!("Stopping replay buffer");
            client.replay_buffer().stop().await?;
            println!("Replay buffer stopped successfully");
        }
        
        Ok(())
    }

    /// Save replay buffer (create clip from buffer)
    pub async fn save_replay_buffer(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.connected {
            return Err("Not connected to OBS WebSocket".into());
        }

        if let Some(client) = &self.client {
            println!("Saving replay buffer clip");
            client.replay_buffer().save().await?;
            println!("Replay buffer clip saved successfully");
        }
        
        Ok(())
    }

    /// Get connection status
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// List available scenes
    pub async fn list_scenes(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        if !self.connected {
            return Err("Not connected to OBS WebSocket".into());
        }

        if let Some(client) = &self.client {
            let scenes = client.scenes().list().await?;
            let scene_names: Vec<String> = scenes.scenes.into_iter().map(|s| format!("{:?}", s.id)).collect();
            return Ok(scene_names);
        }
        
        Err("Client not available".into())
    }
}

/// Simple convenience functions for basic OBS operations
/// These create a controller on demand for one-shot operations

/// Connect to OBS WebSocket with default settings and switch scene
pub async fn connect_and_switch_scene(scene_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut controller = ObsController::new();
    controller.connect("localhost", 4455, None).await?;
    controller.switch_scene(scene_name).await?;
    controller.disconnect().await;
    Ok(())
}

/// Connect to OBS WebSocket and create a buffer clip using replay buffer
pub async fn connect_and_create_buffer_clip() -> Result<(), Box<dyn std::error::Error>> {
    let mut controller = ObsController::new();
    controller.connect("localhost", 4455, None).await?;
    
    // Try to save replay buffer, if it fails start it first
    if let Err(_) = controller.save_replay_buffer().await {
        controller.start_replay_buffer().await?;
        // Wait a moment for buffer to initialize
        sleep(Duration::from_millis(500)).await;
        controller.save_replay_buffer().await?;
    }
    
    controller.disconnect().await;
    Ok(())
}

/// Connect to OBS WebSocket with custom config and switch scene
pub async fn connect_and_switch_scene_with_config(
    host: &str, 
    port: u16, 
    password: Option<&str>, 
    scene_name: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let mut controller = ObsController::new();
    controller.connect(host, port, password).await?;
    controller.switch_scene(scene_name).await?;
    controller.disconnect().await;
    Ok(())
}

/// Legacy API functions for backward compatibility
pub async fn connect_obs() -> Result<(), Box<dyn std::error::Error>> {
    let mut controller = ObsController::new();
    controller.connect("localhost", 4455, None).await?;
    println!("OBS WebSocket connected (legacy mode - auto disconnect)");
    controller.disconnect().await;
    Ok(())
}

pub async fn switch_scene(scene_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    connect_and_switch_scene(scene_name).await
}

pub async fn create_buffer_clip() -> Result<(), Box<dyn std::error::Error>> {
    connect_and_create_buffer_clip().await
}
