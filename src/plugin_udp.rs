use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use log::{info, warn, error, debug};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UdpError {
    #[error("Failed to bind UDP socket: {0}")]
    BindError(#[from] std::io::Error),
    #[error("Protocol parse error: {0}")]
    ParseError(String),
    #[error("Invalid message format: {0}")]
    InvalidFormat(String),
}

#[derive(Debug, Clone)]
pub struct ProtocolDefinition {
    pub main_streams: Vec<String>,
    pub required_arguments: Vec<String>,
    pub optional_arguments: Vec<String>,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct UdpMessage {
    pub stream: String,
    pub arguments: Vec<String>,
    pub raw: String,
}

pub struct UdpServer {
    socket: UdpSocket,
    protocol_definitions: Arc<Mutex<HashMap<String, ProtocolDefinition>>>,
}

impl UdpServer {
    pub async fn new(port: u16) -> Result<Self, UdpError> {
        let addr = format!("0.0.0.0:{}", port);
        let socket = UdpSocket::bind(&addr).await?;
        info!("UDP server bound to {}", addr);
        
        Ok(UdpServer {
            socket,
            protocol_definitions: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub async fn load_protocol_definitions(&self, protocol_file_content: &str) -> Result<(), UdpError> {
        let definitions = parse_protocol_definitions(protocol_file_content)?;
        let mut defs = self.protocol_definitions.lock().await;
        *defs = definitions;
        info!("Loaded {} protocol definitions", defs.len());
        Ok(())
    }

    pub async fn start_listening(&self) -> Result<(), UdpError> {
        info!("Starting UDP server listening loop");
        let mut buf = [0; 1024];
        
        loop {
            match self.socket.recv_from(&mut buf).await {
                Ok((len, addr)) => {
                    let data = &buf[..len];
                    if let Ok(message_str) = std::str::from_utf8(data) {
                        debug!("Received UDP message from {}: {}", addr, message_str);
                        
                        match parse_udp_message(message_str) {
                            Ok(message) => {
                                self.handle_message(message, addr).await;
                            }
                            Err(e) => {
                                warn!("Failed to parse message from {}: {}", addr, e);
                            }
                        }
                    } else {
                        warn!("Received non-UTF8 data from {}", addr);
                    }
                }
                Err(e) => {
                    error!("Error receiving UDP data: {}", e);
                }
            }
        }
    }

    async fn handle_message(&self, message: UdpMessage, addr: SocketAddr) {
        debug!("Handling message: {:?} from {}", message, addr);
        
        // Validate message against protocol definitions
        let defs = self.protocol_definitions.lock().await;
        if let Some(definition) = defs.values().find(|def| {
            def.main_streams.contains(&message.stream)
        }) {
            debug!("Message matches protocol definition for stream: {}", message.stream);
            // Process message according to protocol
            self.process_protocol_message(&message, definition).await;
        } else {
            debug!("No protocol definition found for stream: {}", message.stream);
        }
    }

    async fn process_protocol_message(&self, message: &UdpMessage, _definition: &ProtocolDefinition) {
        info!("Processing {} message with {} arguments", message.stream, message.arguments.len());
        
        match message.stream.as_str() {
            // Points
            "pt1" | "pt2" => self.handle_points_message(message).await,
            // Hit levels
            "hl1" | "hl2" => self.handle_hit_level_message(message).await,
            // Warnings/Gam-jeom
            "wg1" | "wg2" => self.handle_warnings_message(message).await,
            // Injury time
            "ij1" | "ij2" | "ij0" => self.handle_injury_message(message).await,
            // Challenges/IVR
            "ch0" | "ch1" | "ch2" => self.handle_challenge_message(message).await,
            // Break time
            "brk" => self.handle_break_message(message).await,
            // Winner rounds
            "wrd" => self.handle_winner_rounds_message(message).await,
            // Winner
            "wmh" => self.handle_winner_message(message).await,
            // Clock
            "clk" => self.handle_clock_message(message).await,
            // Scores
            stream if stream.starts_with('s') => self.handle_score_message(message).await,
            // Default
            _ => {
                debug!("Unhandled stream type: {}", message.stream);
            }
        }
    }

    async fn handle_points_message(&self, message: &UdpMessage) {
        if let Some(point_type) = message.arguments.first() {
            let athlete = if message.stream == "pt1" { 1 } else { 2 };
            let point_name = match point_type.as_str() {
                "1" => "Punch point",
                "2" => "Body point", 
                "3" => "Head point",
                "4" => "Technical body point",
                "5" => "Technical head point",
                _ => "Unknown point type",
            };
            info!("Athlete {} scored: {}", athlete, point_name);
        }
    }

    async fn handle_hit_level_message(&self, message: &UdpMessage) {
        if let Some(level_str) = message.arguments.first() {
            if let Ok(level) = level_str.parse::<u8>() {
                let athlete = if message.stream == "hl1" { 1 } else { 2 };
                info!("Athlete {} hit level: {}", athlete, level);
            }
        }
    }

    async fn handle_warnings_message(&self, message: &UdpMessage) {
        info!("Warnings/Gam-jeom update: {}", message.raw);
    }

    async fn handle_injury_message(&self, message: &UdpMessage) {
        if let Some(time) = message.arguments.first() {
            let athlete = match message.stream.as_str() {
                "ij1" => "Athlete 1",
                "ij2" => "Athlete 2", 
                "ij0" => "Unidentified athlete",
                _ => "Unknown",
            };
            info!("Injury time for {}: {}", athlete, time);
        }
    }

    async fn handle_challenge_message(&self, message: &UdpMessage) {
        let challenger = match message.stream.as_str() {
            "ch0" => "Referee",
            "ch1" => "Athlete 1",
            "ch2" => "Athlete 2",
            _ => "Unknown",
        };
        info!("Challenge from {}: {:?}", challenger, message.arguments);
    }

    async fn handle_break_message(&self, message: &UdpMessage) {
        if let Some(time) = message.arguments.first() {
            info!("Break time: {}", time);
        }
    }

    async fn handle_winner_rounds_message(&self, message: &UdpMessage) {
        info!("Winner rounds update: {}", message.raw);
    }

    async fn handle_winner_message(&self, message: &UdpMessage) {
        if let Some(winner_name) = message.arguments.first() {
            let empty_string = String::new();
            let classification = message.arguments.get(1).unwrap_or(&empty_string);
            info!("Winner: {} {}", winner_name, classification);
        }
    }

    async fn handle_clock_message(&self, message: &UdpMessage) {
        if let Some(time) = message.arguments.first() {
            let empty_string = String::new();
            let action = message.arguments.get(1).unwrap_or(&empty_string);
            info!("Clock: {} {}", time, action);
        }
    }

    async fn handle_score_message(&self, message: &UdpMessage) {
        info!("Score update {}: {:?}", message.stream, message.arguments);
    }
}

pub fn parse_protocol_definitions(content: &str) -> Result<HashMap<String, ProtocolDefinition>, UdpError> {
    let mut definitions = HashMap::new();
    let sections: Vec<&str> = content.split("---").collect();
    
    for section in sections {
        if let Ok(definition) = parse_protocol_section(section) {
            // Use the first main stream as the key
            if let Some(first_stream) = definition.main_streams.first() {
                let key = first_stream.split(';').next().unwrap_or(first_stream).to_string();
                definitions.insert(key, definition);
            }
        }
    }
    
    Ok(definitions)
}

pub fn parse_protocol_section(section: &str) -> Result<ProtocolDefinition, UdpError> {
    let mut main_streams = Vec::new();
    let mut required_arguments = Vec::new();
    let mut optional_arguments = Vec::new();
    let mut examples = Vec::new();
    
    let mut current_type = "";
    
    for line in section.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if line == "MAIN_STREAMS:" {
            current_type = "main_streams";
        } else if line == "REQUIRED_ARGUMENTS:" {
            current_type = "required_arguments";
        } else if line == "OPTIONAL_ARGUMENTS:" {
            current_type = "optional_arguments";
        } else if line == "EXAMPLES:" {
            current_type = "examples";
        } else {
            match current_type {
                "main_streams" => {
                    if let Some(stream) = line.split(';').next() {
                        main_streams.push(stream.trim().to_string());
                    }
                }
                "required_arguments" => {
                    if let Some(arg) = line.split(';').next() {
                        required_arguments.push(arg.trim().to_string());
                    }
                }
                "optional_arguments" => {
                    if let Some(arg) = line.split(';').next() {
                        optional_arguments.push(arg.trim().to_string());
                    }
                }
                "examples" => {
                    examples.push(line.to_string());
                }
                _ => {}
            }
        }
    }
    
    Ok(ProtocolDefinition {
        main_streams,
        required_arguments,
        optional_arguments,
        examples,
    })
}

pub fn parse_udp_message(message: &str) -> Result<UdpMessage, UdpError> {
    let message = message.trim();
    
    if message.is_empty() {
        return Err(UdpError::InvalidFormat("Empty message".to_string()));
    }
    
    let parts: Vec<&str> = message.split(';').collect();
    if parts.is_empty() {
        return Err(UdpError::InvalidFormat("No stream specified".to_string()));
    }
    
    let stream = parts[0].to_string();
    let arguments: Vec<String> = parts[1..].iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    
    Ok(UdpMessage {
        stream,
        arguments,
        raw: message.to_string(),
    })
}

// Public API function
pub async fn start_udp_server() -> Result<(), UdpError> {
    let server = UdpServer::new(6000).await?;
    
    // Load protocol definitions from file
    let protocol_content = include_str!("../protocol/pss_schema.txt");
    server.load_protocol_definitions(protocol_content).await?;
    
    info!("UDP server starting on port 6000");
    server.start_listening().await
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_udp_message() {
        // Test point message
        let msg = parse_udp_message("pt1;3;").unwrap();
        assert_eq!(msg.stream, "pt1");
        assert_eq!(msg.arguments, vec!["3"]);
        
        // Test hit level message
        let msg = parse_udp_message("hl1;75;").unwrap();
        assert_eq!(msg.stream, "hl1");
        assert_eq!(msg.arguments, vec!["75"]);
        
        // Test complex message
        let msg = parse_udp_message("wg1;1;wg2;2;").unwrap();
        assert_eq!(msg.stream, "wg1");
        assert_eq!(msg.arguments, vec!["1", "wg2", "2"]);
        
        // Test message with multiple arguments
        let msg = parse_udp_message("ij1;1:23;show;").unwrap();
        assert_eq!(msg.stream, "ij1");
        assert_eq!(msg.arguments, vec!["1:23", "show"]);
        
        // Test empty message
        assert!(parse_udp_message("").is_err());
    }
    
    #[test]
    fn test_parse_protocol_definitions() {
        let protocol_content = r#"
# POINTS
# Stream broadcasted when points are added.

MAIN_STREAMS:
  pt1;  Main stream for athlete 1
  pt2;  Main stream for athlete 2

REQUIRED_ARGUMENTS:
  1;  Punch point
  2;  Body point

EXAMPLES:
  pt1;1;
  pt2;2;

---

# HITLEVEL
# Stream broadcasted when hit happens.

MAIN_STREAMS:
  hl1;  Main stream for athlete 1
  hl2;  Main stream for athlete 2

REQUIRED_ARGUMENTS:
  50;  Hit Level value (from 1 to 100)

EXAMPLES:
  hl1;50;
"#;
        
        let definitions = parse_protocol_definitions(protocol_content).unwrap();
        assert_eq!(definitions.len(), 2);
        
        // Check points definition
        assert!(definitions.contains_key("pt1"));
        let pt_def = definitions.get("pt1").unwrap();
        assert_eq!(pt_def.main_streams, vec!["pt1", "pt2"]);
        assert_eq!(pt_def.required_arguments, vec!["1", "2"]);
        
        // Check hit level definition  
        assert!(definitions.contains_key("hl1"));
        let hl_def = definitions.get("hl1").unwrap();
        assert_eq!(hl_def.main_streams, vec!["hl1", "hl2"]);
        assert_eq!(hl_def.required_arguments, vec!["50"]);
    }
}
