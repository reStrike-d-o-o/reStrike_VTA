#[cfg(test)]
mod tests {
    use crate::plugin_store::{Database, Match, Recording, Event};
    use chrono::Utc;
    use tempfile::NamedTempFile;

    fn create_test_db() -> (Database, NamedTempFile) {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let db = Database::new(db_path).unwrap();
        (db, temp_file)
    }

    #[test]
    fn test_match_crud() {
        let (db, _temp_file) = create_test_db();
        
        // Create a match
        let match_data = Match {
            id: None,
            name: "Test Match".to_string(),
            map: "de_mirage".to_string(),
            team1: "Team A".to_string(),
            team2: "Team B".to_string(),
            date: Utc::now(),
            status: "live".to_string(),
        };
        
        let match_id = db.create_match(&match_data).unwrap();
        assert!(match_id > 0);
        
        // Read the match
        let retrieved_match = db.get_match(match_id).unwrap().unwrap();
        assert_eq!(retrieved_match.name, "Test Match");
        assert_eq!(retrieved_match.map, "de_mirage");
        
        // Update the match
        let mut updated_match = retrieved_match;
        updated_match.status = "finished".to_string();
        db.update_match(match_id, &updated_match).unwrap();
        
        // Verify update
        let updated_retrieved = db.get_match(match_id).unwrap().unwrap();
        assert_eq!(updated_retrieved.status, "finished");
        
        // Delete the match
        db.delete_match(match_id).unwrap();
        let deleted_match = db.get_match(match_id).unwrap();
        assert!(deleted_match.is_none());
    }

    #[test]
    fn test_recording_crud() {
        let (db, _temp_file) = create_test_db();
        
        // First create a match
        let match_data = Match {
            id: None,
            name: "Test Match".to_string(),
            map: "de_inferno".to_string(),
            team1: "Team C".to_string(),
            team2: "Team D".to_string(),
            date: Utc::now(),
            status: "finished".to_string(),
        };
        let match_id = db.create_match(&match_data).unwrap();
        
        // Create a recording
        let recording = Recording {
            id: None,
            match_id,
            file_path: "/test/recording.mp4".to_string(),
            start_time: Utc::now(),
            end_time: Some(Utc::now()),
            size_bytes: 1024,
            is_highlight: true,
        };
        
        let recording_id = db.create_recording(&recording).unwrap();
        assert!(recording_id > 0);
        
        // Read the recording
        let retrieved_recording = db.get_recording(recording_id).unwrap().unwrap();
        assert_eq!(retrieved_recording.file_path, "/test/recording.mp4");
        assert_eq!(retrieved_recording.is_highlight, true);
        
        // Get recordings for match
        let recordings = db.get_recordings_for_match(match_id).unwrap();
        assert_eq!(recordings.len(), 1);
    }

    #[test]
    fn test_event_crud() {
        let (db, _temp_file) = create_test_db();
        
        // First create a match
        let match_data = Match {
            id: None,
            name: "Event Test Match".to_string(),
            map: "de_cache".to_string(),
            team1: "Team E".to_string(),
            team2: "Team F".to_string(),
            date: Utc::now(),
            status: "live".to_string(),
        };
        let match_id = db.create_match(&match_data).unwrap();
        
        // Create an event
        let event = Event {
            id: None,
            match_id,
            event_type: "kill".to_string(),
            timestamp: Utc::now(),
            player: Some("player1".to_string()),
            details: "Headshot with AK-47".to_string(),
        };
        
        let event_id = db.create_event(&event).unwrap();
        assert!(event_id > 0);
        
        // Read the event
        let retrieved_event = db.get_event(event_id).unwrap().unwrap();
        assert_eq!(retrieved_event.event_type, "kill");
        assert_eq!(retrieved_event.player, Some("player1".to_string()));
        
        // Get events for match
        let events = db.get_events_for_match(match_id).unwrap();
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_database_initialization() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        
        // Creating database should not fail
        let db = Database::new(db_path).unwrap();
        
        // Tables should be accessible (no errors when querying)
        let matches = db.get_all_matches().unwrap();
        assert_eq!(matches.len(), 0);
    }
}