use crate::ip_address::IPAddress;
use crate::protocol::Protocol;

#[derive(Clone)]

pub struct ParsedPacket {
    pub link :String,
    pub ip :IPAddress,
    pub protocol :Protocol,
    pub transport :String,
    pub payload :String,
}

impl ParsedPacket {
    pub fn new(link: String, ip: IPAddress, protocol: Protocol, transport: String, payload: String) -> Self {
        Self {
            link: link.to_string(),
            ip: ip,
            protocol: protocol,
            transport: transport.to_string(),
            payload: payload.to_string(),
        }
    }

    pub fn print(&self) {
        println!("_______________________________");
        self.ip.print();
        self.protocol.print();
        println!("Payload: {}", self.payload);
        println!("_______________________________");

    }


}