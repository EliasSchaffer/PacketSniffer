mod parsed_packet;
mod protocol;
mod ip_address;

use std::collections::LinkedList;
use std::process::exit;
use pcap::Device;
use inquire::Select;
use etherparse::{NetSlice, SlicedPacket};
use crate::ip_address::IPAddress;
use crate::parsed_packet::ParsedPacket;
use crate::protocol::Protocol;
use std::fmt::Write;


fn main() {
    let mut selected_device:Device = Device::lookup().unwrap().unwrap();
    let devices = Device::list().unwrap();
    let  device_names = devices
        .iter()
        .map(|device| device.desc.clone().unwrap_or(device.name.clone()))
        .collect::<Vec<String>>();
    let ans = Select::new("Choose a network port:", device_names.clone()).prompt();
    let mut packet_list: LinkedList<ParsedPacket> = LinkedList::new();

    match ans {
        Ok(choice) => {
            let index = device_names.iter().position(|f| *f == choice);

            match index {
                Some(i) => selected_device = devices[i].clone(),
                None => {
                    println!("Error getting Device from List");
                    exit(1)
                }
            }
        },
        Err(_) => println!("Selection canceled"),
    }

    let mut capture = selected_device.open().unwrap();

    while let Ok(packet) = capture.next_packet() {
        print_and_safe_packet(&packet, &mut packet_list);
    }
}

fn print_and_safe_packet(packet: &pcap::Packet, packet_list: &mut LinkedList<ParsedPacket>) {

    if let Ok(sliced) = SlicedPacket::from_ethernet(packet.data) {

        let mut temp_link = String::new();
        let mut temp_ip = IPAddress::new_empty();
        let mut temp_transport = String::new();
        let mut temp_payload = String::new();
        let mut temp_protocol = Protocol::new_empty();

        if let Some(link) = sliced.link {
            temp_link = format!("{:?}", link);
        }

        if let Some(net) = sliced.net {
            temp_ip = parse_ip(net);
        }

        if let Some(transport) = sliced.transport {
            temp_transport = format!("{:?}", transport);

            match transport {
                etherparse::TransportSlice::Tcp(tcp) => {
                    temp_protocol = Protocol::new("TCP", tcp.source_port(), tcp.destination_port());
                    temp_payload = parse_payload(tcp.payload());
                }

                etherparse::TransportSlice::Udp(udp) => {
                    temp_protocol = Protocol::new("UDP", udp.source_port(), udp.destination_port());
                    temp_payload = parse_payload(udp.payload());

                }
                _ => {}
            }
        }
        let parsed_packet = ParsedPacket::new(temp_link, temp_ip, temp_protocol, temp_transport, temp_payload);
        packet_list.push_back(parsed_packet.clone());
        parsed_packet.print();
    }
}

fn parse_payload(payload: &[u8]) -> String {
    let mut s = String::new();

    if payload.is_empty() {
        write!(&mut s, "Payload: <empty>").unwrap();
        return s;
    }

    // HEX part
    write!(&mut s, "Payload HEX: ").unwrap();
    for b in payload {
        write!(&mut s, "{:02x} ", b).unwrap();
    }

    // ASCII part
    write!(&mut s, " | ASCII: ").unwrap();
    for b in payload {
        if b.is_ascii_graphic() || *b == b' ' {
            s.push(*b as char);
        } else {
            s.push('.');
        }
    }

    s
}

fn parse_ip(net: NetSlice) -> IPAddress {
    let mut ipv4_source= String::new();
    let mut ipv4_destination= String::new();
    let mut ipv6_source= String::new();
    let mut ipv6_destination= String::new();
    match net {
        etherparse::NetSlice::Ipv4(ipv4) => {

            ipv4_source = ipv4.header().source_addr().to_string();
            ipv4_destination = ipv4.header().destination_addr().to_string();


        }
        etherparse::NetSlice::Ipv6(ipv6) => {
            ipv6_source = ipv6.header().source_addr().to_string();
            ipv6_destination = ipv6.header().destination_addr().to_string();
        }
    }
    return IPAddress::new(&ipv4_source, &ipv4_destination, &ipv6_source, &ipv6_destination)
}
