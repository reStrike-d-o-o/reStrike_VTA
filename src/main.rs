mod plugin_store;

#[cfg(test)]
#[path = "plugin_store_tests.rs"]
mod plugin_store_tests;

use plugin_store::{Database, Match, Recording, Event};
use chrono::Utc;

fn main() {
    println!("reStrike VTA backend starting...");
    
    // Example usage of the database
    match initialize_database() {
        Ok(_) => println!("Database initialized successfully"),
        Err(e) => eprintln!("Error initializing database: {}", e),
    }
}

fn initialize_database() -> Result<(), Box<dyn std::error::Error>> {
    // Create database connection
    let db = Database::new("reStrike_VTA.db")?;
    
    // Example: Create a sample match
    let sample_match = Match {
        id: None,
        name: "Championship Final".to_string(),
        map: "de_dust2".to_string(),
        team1: "Team Alpha".to_string(),
        team2: "Team Beta".to_string(),
        date: Utc::now(),
        status: "upcoming".to_string(),
    };
    
    let match_id = db.create_match(&sample_match)?;
    println!("Created match with ID: {}", match_id);
    
    // Example: Create a sample recording
    let sample_recording = Recording {
        id: None,
        match_id,
        file_path: "/recordings/match_001.mp4".to_string(),
        start_time: Utc::now(),
        end_time: None,
        size_bytes: 1024 * 1024 * 500, // 500MB
        is_highlight: false,
    };
    
    let recording_id = db.create_recording(&sample_recording)?;
    println!("Created recording with ID: {}", recording_id);
    
    // Example: Create a sample event
    let sample_event = Event {
        id: None,
        match_id,
        event_type: "round_start".to_string(),
        timestamp: Utc::now(),
        player: None,
        details: "Round 1 started".to_string(),
    };
    
    let event_id = db.create_event(&sample_event)?;
    println!("Created event with ID: {}", event_id);
    
    // Test retrieval
    if let Some(retrieved_match) = db.get_match(match_id)? {
        println!("Retrieved match: {:?}", retrieved_match);
    }
    
    Ok(())
}
