mod parsed_packet;
mod protocol;
mod ip_address;

use std::collections::LinkedList;
use std::process::exit;
use pcap::Device;
use inquire::{Confirm, Select};
use etherparse::{NetSlice, SlicedPacket};
use crate::ip_address::IPAddress;
use crate::parsed_packet::ParsedPacket;
use crate::protocol::Protocol;
use std::fs::{File, OpenOptions};
use std::io::Write;
use chrono::Local;
use std::io::Write as IoWrite;
use std::fmt::Write as FmtWrite;

fn main() {
    let mut selected_device:Device = Device::lookup().unwrap().unwrap();
    let devices = Device::list().unwrap();
    let  device_names = devices
        .iter()
        .map(|device| device.desc.clone().unwrap_or(device.name.clone()))
        .collect::<Vec<String>>();
    let ans = Select::new("Choose a network port:", device_names.clone()).prompt();
    let mut packet_list: LinkedList<ParsedPacket> = LinkedList::new();
    let mut safe_to_file = false;

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

    let answer = Confirm::new("Do you want to safe your Session in a File?")
        .with_default(true)
        .prompt()
        .unwrap();

    let log_filename = if answer {
        safe_to_file = true;
        let now = Local::now();
        format!(
            "packets_log_{}.txt",
            now.format("%Y-%m-%d_%H-%M-%S")
        )
    } else {
        String::new()
    };
    let mut capture = selected_device.open().unwrap();

    while let Ok(packet) = capture.next_packet() {
        print_and_safe_packet(&packet, &mut packet_list, safe_to_file, &log_filename);
    }
}

fn print_and_safe_packet(packet: &pcap::Packet, packet_list: &mut LinkedList<ParsedPacket>, safe_to_file: bool, log_filename: &str) {

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
        if safe_to_file {
            save_to_file(&parsed_packet, log_filename);
        }
    }
}

fn save_to_file(parsed_packet: &ParsedPacket, filename: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
        .unwrap();

    writeln!(file, "{}\n", parsed_packet).unwrap();
}

fn parse_payload(payload: &[u8]) -> String {
    use std::fmt::Write as FmtWrite;

    let mut s = String::new();

    if payload.is_empty() {
        write!(&mut s, "Payload: <empty>").unwrap();
        return s;
    }

    write!(&mut s, "Payload HEX: ").unwrap();

    for b in payload {
        write!(&mut s, "{:02x} ", b).unwrap();
    }

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
