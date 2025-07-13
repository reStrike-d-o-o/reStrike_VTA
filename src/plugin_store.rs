use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use anyhow::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Match {
    pub id: Option<i64>,
    pub name: String,
    pub map: String,
    pub team1: String,
    pub team2: String,
    pub date: DateTime<Utc>,
    pub status: String, // e.g., "upcoming", "live", "finished"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recording {
    pub id: Option<i64>,
    pub match_id: i64,
    pub file_path: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub size_bytes: i64,
    pub is_highlight: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: Option<i64>,
    pub match_id: i64,
    pub event_type: String, // e.g., "kill", "bomb_plant", "round_end"
    pub timestamp: DateTime<Utc>,
    pub player: Option<String>,
    pub details: String, // JSON or text details
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self, Error> {
        let conn = Connection::open(db_path)?;
        let db = Database { conn };
        db.create_tables()?;
        Ok(db)
    }

    fn create_tables(&self) -> Result<(), Error> {
        // Create matches table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS matches (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                map TEXT NOT NULL,
                team1 TEXT NOT NULL,
                team2 TEXT NOT NULL,
                date TEXT NOT NULL,
                status TEXT NOT NULL
            )",
            [],
        )?;

        // Create recordings table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS recordings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                match_id INTEGER NOT NULL,
                file_path TEXT NOT NULL,
                start_time TEXT NOT NULL,
                end_time TEXT,
                size_bytes INTEGER NOT NULL,
                is_highlight BOOLEAN NOT NULL,
                FOREIGN KEY (match_id) REFERENCES matches (id)
            )",
            [],
        )?;

        // Create events table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                match_id INTEGER NOT NULL,
                event_type TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                player TEXT,
                details TEXT NOT NULL,
                FOREIGN KEY (match_id) REFERENCES matches (id)
            )",
            [],
        )?;

        Ok(())
    }

    // Match CRUD operations
    pub fn create_match(&self, match_data: &Match) -> Result<i64, Error> {
        self.conn.execute(
            "INSERT INTO matches (name, map, team1, team2, date, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                match_data.name,
                match_data.map,
                match_data.team1,
                match_data.team2,
                match_data.date.to_rfc3339(),
                match_data.status
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_match(&self, id: i64) -> Result<Option<Match>, Error> {
        let mut stmt = self.conn.prepare("SELECT id, name, map, team1, team2, date, status FROM matches WHERE id = ?1")?;
        let mut rows = stmt.query_map([id], |row| {
            let date_str: String = row.get(5)?;
            Ok(Match {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                map: row.get(2)?,
                team1: row.get(3)?,
                team2: row.get(4)?,
                date: DateTime::parse_from_rfc3339(&date_str).unwrap().with_timezone(&Utc),
                status: row.get(6)?,
            })
        })?;

        match rows.next() {
            Some(Ok(match_data)) => Ok(Some(match_data)),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    pub fn get_all_matches(&self) -> Result<Vec<Match>, Error> {
        let mut stmt = self.conn.prepare("SELECT id, name, map, team1, team2, date, status FROM matches ORDER BY date DESC")?;
        let rows = stmt.query_map([], |row| {
            let date_str: String = row.get(5)?;
            Ok(Match {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                map: row.get(2)?,
                team1: row.get(3)?,
                team2: row.get(4)?,
                date: DateTime::parse_from_rfc3339(&date_str).unwrap().with_timezone(&Utc),
                status: row.get(6)?,
            })
        })?;

        let mut matches = Vec::new();
        for row in rows {
            matches.push(row?);
        }
        Ok(matches)
    }

    pub fn update_match(&self, id: i64, match_data: &Match) -> Result<(), Error> {
        self.conn.execute(
            "UPDATE matches SET name = ?1, map = ?2, team1 = ?3, team2 = ?4, date = ?5, status = ?6 WHERE id = ?7",
            params![
                match_data.name,
                match_data.map,
                match_data.team1,
                match_data.team2,
                match_data.date.to_rfc3339(),
                match_data.status,
                id
            ],
        )?;
        Ok(())
    }

    pub fn delete_match(&self, id: i64) -> Result<(), Error> {
        self.conn.execute("DELETE FROM matches WHERE id = ?1", [id])?;
        Ok(())
    }

    // Recording CRUD operations
    pub fn create_recording(&self, recording: &Recording) -> Result<i64, Error> {
        let end_time_str = recording.end_time.map(|dt| dt.to_rfc3339());
        self.conn.execute(
            "INSERT INTO recordings (match_id, file_path, start_time, end_time, size_bytes, is_highlight) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                recording.match_id,
                recording.file_path,
                recording.start_time.to_rfc3339(),
                end_time_str,
                recording.size_bytes,
                recording.is_highlight
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_recording(&self, id: i64) -> Result<Option<Recording>, Error> {
        let mut stmt = self.conn.prepare("SELECT id, match_id, file_path, start_time, end_time, size_bytes, is_highlight FROM recordings WHERE id = ?1")?;
        let mut rows = stmt.query_map([id], |row| {
            let start_time_str: String = row.get(3)?;
            let end_time_str: Option<String> = row.get(4)?;
            let end_time = end_time_str.map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc));
            
            Ok(Recording {
                id: Some(row.get(0)?),
                match_id: row.get(1)?,
                file_path: row.get(2)?,
                start_time: DateTime::parse_from_rfc3339(&start_time_str).unwrap().with_timezone(&Utc),
                end_time,
                size_bytes: row.get(5)?,
                is_highlight: row.get(6)?,
            })
        })?;

        match rows.next() {
            Some(Ok(recording)) => Ok(Some(recording)),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    pub fn get_recordings_for_match(&self, match_id: i64) -> Result<Vec<Recording>, Error> {
        let mut stmt = self.conn.prepare("SELECT id, match_id, file_path, start_time, end_time, size_bytes, is_highlight FROM recordings WHERE match_id = ?1 ORDER BY start_time DESC")?;
        let rows = stmt.query_map([match_id], |row| {
            let start_time_str: String = row.get(3)?;
            let end_time_str: Option<String> = row.get(4)?;
            let end_time = end_time_str.map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc));
            
            Ok(Recording {
                id: Some(row.get(0)?),
                match_id: row.get(1)?,
                file_path: row.get(2)?,
                start_time: DateTime::parse_from_rfc3339(&start_time_str).unwrap().with_timezone(&Utc),
                end_time,
                size_bytes: row.get(5)?,
                is_highlight: row.get(6)?,
            })
        })?;

        let mut recordings = Vec::new();
        for row in rows {
            recordings.push(row?);
        }
        Ok(recordings)
    }

    pub fn update_recording(&self, id: i64, recording: &Recording) -> Result<(), Error> {
        let end_time_str = recording.end_time.map(|dt| dt.to_rfc3339());
        self.conn.execute(
            "UPDATE recordings SET match_id = ?1, file_path = ?2, start_time = ?3, end_time = ?4, size_bytes = ?5, is_highlight = ?6 WHERE id = ?7",
            params![
                recording.match_id,
                recording.file_path,
                recording.start_time.to_rfc3339(),
                end_time_str,
                recording.size_bytes,
                recording.is_highlight,
                id
            ],
        )?;
        Ok(())
    }

    pub fn delete_recording(&self, id: i64) -> Result<(), Error> {
        self.conn.execute("DELETE FROM recordings WHERE id = ?1", [id])?;
        Ok(())
    }

    // Event CRUD operations
    pub fn create_event(&self, event: &Event) -> Result<i64, Error> {
        self.conn.execute(
            "INSERT INTO events (match_id, event_type, timestamp, player, details) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                event.match_id,
                event.event_type,
                event.timestamp.to_rfc3339(),
                event.player,
                event.details
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_event(&self, id: i64) -> Result<Option<Event>, Error> {
        let mut stmt = self.conn.prepare("SELECT id, match_id, event_type, timestamp, player, details FROM events WHERE id = ?1")?;
        let mut rows = stmt.query_map([id], |row| {
            let timestamp_str: String = row.get(3)?;
            Ok(Event {
                id: Some(row.get(0)?),
                match_id: row.get(1)?,
                event_type: row.get(2)?,
                timestamp: DateTime::parse_from_rfc3339(&timestamp_str).unwrap().with_timezone(&Utc),
                player: row.get(4)?,
                details: row.get(5)?,
            })
        })?;

        match rows.next() {
            Some(Ok(event)) => Ok(Some(event)),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    pub fn get_events_for_match(&self, match_id: i64) -> Result<Vec<Event>, Error> {
        let mut stmt = self.conn.prepare("SELECT id, match_id, event_type, timestamp, player, details FROM events WHERE match_id = ?1 ORDER BY timestamp ASC")?;
        let rows = stmt.query_map([match_id], |row| {
            let timestamp_str: String = row.get(3)?;
            Ok(Event {
                id: Some(row.get(0)?),
                match_id: row.get(1)?,
                event_type: row.get(2)?,
                timestamp: DateTime::parse_from_rfc3339(&timestamp_str).unwrap().with_timezone(&Utc),
                player: row.get(4)?,
                details: row.get(5)?,
            })
        })?;

        let mut events = Vec::new();
        for row in rows {
            events.push(row?);
        }
        Ok(events)
    }

    pub fn update_event(&self, id: i64, event: &Event) -> Result<(), Error> {
        self.conn.execute(
            "UPDATE events SET match_id = ?1, event_type = ?2, timestamp = ?3, player = ?4, details = ?5 WHERE id = ?6",
            params![
                event.match_id,
                event.event_type,
                event.timestamp.to_rfc3339(),
                event.player,
                event.details,
                id
            ],
        )?;
        Ok(())
    }

    pub fn delete_event(&self, id: i64) -> Result<(), Error> {
        self.conn.execute("DELETE FROM events WHERE id = ?1", [id])?;
        Ok(())
    }
}
