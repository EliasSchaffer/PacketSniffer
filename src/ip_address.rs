#[derive(Clone)]
pub struct IPAddress {
    ipv4_source :String,
    ipv4_destination :String,
    ipv6_source :String,
    ipv6_destination :String,
}

impl IPAddress {

    pub fn new(ipv4_source: &str, ipv4_destination: &str, ipv6_source: &str, ipv6_destination: &str) -> Self {
        Self {
            ipv4_source: ipv4_source.to_string(),
            ipv4_destination: ipv4_destination.to_string(),
            ipv6_source: ipv6_source.to_string(),
            ipv6_destination: ipv6_destination.to_string(),
        }
    }
    
    pub fn new_empty() -> Self {
        Self {
            ipv4_source: String::new(),
            ipv4_destination: String::new(),
            ipv6_source: String::new(),
            ipv6_destination: String::new(),
        }  
    }
    
    
    pub fn print(&self) {
        println!("IPv4 Source: {}", self.ipv4_source);
        println!("IPv4 Destination: {}", self.ipv4_destination);
        println!("IPv6 Source: {}", self.ipv6_source);
        println!("IPv6 Destination: {}", self.ipv6_destination);
    }
}