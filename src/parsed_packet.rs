use crate::ip_address::IPAddress;
use crate::protocol::Protocol;
use std::fmt::{self, Display, Formatter};
use crate::payload::Payload;

#[derive(Clone)]

pub struct ParsedPacket {
    pub link :String,
    pub ip :IPAddress,
    pub protocol :Protocol,
    pub transport :String,
    pub payload :Payload,
}

impl Display for ParsedPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "_______________________________\n{} | {}\n{}\n_______________________________",
            self.ip, self.protocol, self.payload
        )
    }
}

impl ParsedPacket {
    pub fn new(link: String, ip: IPAddress, protocol: Protocol, transport: String, payload: Payload) -> Self {
        Self {
            link: link.to_string(),
            ip: ip,
            protocol: protocol,
            transport: transport.to_string(),
            payload: payload,
        }
    }

    pub fn print(&self) {
        println!("_______________________________");
        self.ip.print();
        self.protocol.print();
        println!("{}", self.payload);
        println!("_______________________________");

    }
}

