use crate::parsed_packet::ParsedPacket;

#[derive(Clone)]
pub struct Protocol {
    protocol: String,
    source_port:String,
    destination_port:String,
}

impl Protocol {

    pub fn new(protocol: &str, source: u16, destination: u16) -> Self {
        Self {
            protocol: protocol.to_string(),
            source_port: source.to_string(),
            destination_port: destination.to_string(),
        }
    }

    pub fn new_empty() -> Self {
        Self {
            protocol: String::new(),
            source_port: String::new(),
            destination_port: String::new(),
        }
    }
    
    pub fn print(&self) {
        println!("Protocol: {}", self.protocol);
        println!("Source Port: {}", self.source_port);
        println!("Destination Port: {}", self.destination_port);
    }
}