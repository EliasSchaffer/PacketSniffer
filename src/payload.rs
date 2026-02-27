#[derive(Clone)]

pub struct Payload {
    hex: String,
    ascii: String,
}

impl std::fmt::Display for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Payload HEX: {} | ASCII: {}", self.hex, self.ascii)
    }
}
impl Payload {
    pub fn new(hex: String, ascii: String) -> Payload {
        Payload { hex, ascii 
        }
    }
    pub fn new_empty() -> Payload {
        Payload { hex: String::new(), ascii: String::new() }
    }
}