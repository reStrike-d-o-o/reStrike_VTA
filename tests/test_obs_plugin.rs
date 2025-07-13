use restrike_vta::plugin_obs::{ObsController, connect_and_switch_scene, create_buffer_clip};

#[tokio::test]
async fn test_obs_controller_creation() {
    let controller = ObsController::new();
    assert!(!controller.is_connected());
}

#[tokio::test]
async fn test_scene_switch_without_connection() {
    // This should fail gracefully when OBS is not running
    let result = connect_and_switch_scene("Test Scene").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_buffer_clip_without_connection() {
    // This should fail gracefully when OBS is not running
    let result = create_buffer_clip().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_controller_connection_attempt() {
    let mut controller = ObsController::new();
    
    // This should fail gracefully when OBS is not running
    let result = controller.connect("localhost", 4455, None).await;
    assert!(result.is_err());
    assert!(!controller.is_connected());
}