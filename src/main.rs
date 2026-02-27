mod parsed_packet;
mod protocol;
mod ip_address;
mod payload;

use std::collections::LinkedList;
use std::process::exit;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use pcap::Device;
use inquire::{Select};
use etherparse::{NetSlice, SlicedPacket};
use chrono::Local;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crate::ip_address::IPAddress;
use crate::parsed_packet::ParsedPacket;
use crate::protocol::Protocol;
use crate::payload::Payload;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut selected_device:Device = Device::lookup().unwrap().unwrap();
    let devices = Device::list().unwrap();
    let  device_names = devices
        .iter()
        .map(|device| device.desc.clone().unwrap_or(device.name.clone()))
        .collect::<Vec<String>>();
    let ans = Select::new("Choose a network port:", device_names.clone()).prompt();
    let packet_list: Arc<Mutex<LinkedList<ParsedPacket>>> = Arc::new(Mutex::new(LinkedList::new()));
    let mut sessions_list: LinkedList<LinkedList<ParsedPacket>> = LinkedList::new();
    let menu_options = vec![
        "Start new Session",
        "Show Sessions",
        "Save Sessions",
        "Exit",
    ];
    let end_capture = Arc::new(AtomicBool::new(false));

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

    while true {
        let ans = Select::new("Menu:", menu_options.clone()).prompt();        match ans {
            Ok(choice) => {
                match choice {
                    "Start new Session" => {
                        end_capture.store(true, Ordering::Relaxed);
                        let packet_list_clone = Arc::clone(&packet_list);
                        let end_capture_clone = Arc::clone(&end_capture);
                        let end_capture_wait = Arc::clone(&end_capture);
                        let device_clone = selected_device.clone();

                        thread::spawn(move || {
                            start_session(
                                device_clone,
                                packet_list_clone,
                                end_capture_clone,
                            );
                        });

                        println!("Press SPACE to stop...");
                        
                        while end_capture_wait.load(Ordering::Relaxed) {
                            thread::sleep(std::time::Duration::from_millis(100));
                        }
                        
                        let captured_packets = packet_list.lock().unwrap().clone();
                        if !captured_packets.is_empty() {
                            sessions_list.push_back(captured_packets);
                            println!("Session saved with {} packets", sessions_list.back().unwrap().len());
                        }
                        packet_list.lock().unwrap().clear();
                    }

                    "Show Sessions" => {
                        println!("Showing sessions...");
                        show_sessions(sessions_list.clone());
                    }

                    "Save Sessions" => {
                        save_session(sessions_list.clone());
                    }

                    "Exit" => {
                        println!("Exiting...");
                        break;
                    }

                    _ => {}
                }
            }

            Err(_) => println!("Selection canceled"),
        }

    }
}

fn print_and_safe_packet(packet: &pcap::Packet, packet_list: &mut LinkedList<ParsedPacket>) {

    if let Ok(sliced) = SlicedPacket::from_ethernet(packet.data) {

        let mut temp_link = String::new();
        let mut temp_ip = IPAddress::new_empty();
        let mut temp_transport = String::new();
        let mut temp_payload = Payload::new_empty();
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

fn save_to_file(parsed_packet: &ParsedPacket, filename: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
        .unwrap();

    writeln!(file, "{}\n", parsed_packet).unwrap();
}

fn parse_payload(payload: &[u8]) -> Payload {
    use std::fmt::Write as FmtWrite;

    let mut hex = String::new();
    let mut ascii = String::new();

    if payload.is_empty() {
        hex = "<empty>".to_string();
        ascii = "<empty>".to_string();
        return Payload::new(hex, ascii);
    }

    for b in payload {
        write!(hex, "{:02x} ", b).unwrap();
    }

    for b in payload {
        if b.is_ascii_graphic() || *b == b' ' {
            ascii.push(*b as char);
        } else {
            ascii.push('.');
        }
    }

    Payload::new(hex, ascii)
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


fn start_session(selected_device: Device, packet_list: Arc<Mutex<LinkedList<ParsedPacket>>>, running: Arc<AtomicBool>) {
    enable_raw_mode().unwrap();
    let mut capture = pcap::Capture::from_device(selected_device)
        .unwrap()
        .timeout(50)
        .open()
        .unwrap();

    loop {
        if event::poll(std::time::Duration::from_millis(10)).unwrap() {
            if let Event::Key(KeyEvent { code: KeyCode::Char(' '), .. }) = event::read().unwrap() {
                println!("SPACE pressed, stopping capture...");
                running.store(false, Ordering::Relaxed);
                break;
            }
        }

        match capture.next_packet() {
            Ok(packet) => {
                let mut list = packet_list.lock().unwrap();
                print_and_safe_packet(&packet, &mut list);
            }
            Err(pcap::Error::TimeoutExpired) => {}
            Err(_) => break,
        }
    }

    disable_raw_mode().unwrap();
    println!("Session ended.");
}

fn show_sessions(sessions_list: LinkedList<LinkedList<ParsedPacket>>) {
    if sessions_list.is_empty() {
        println!("No sessions captured yet.");
        return;
    }
    
    let mut index = 0;
    for session in sessions_list {
        println!("\n========== Session {} ({} packets) ==========", index, session.len());

        index += 1;
    }
    println!("\nTotal sessions: {}", index);
}

fn save_session(sessions_list: LinkedList<LinkedList<ParsedPacket>>) {
    if sessions_list.is_empty() {
        println!("No sessions captured yet.");
        return;
    }
    let now = Local::now();
    let filename = format!("Session{}.txt", now.format("%Y-%m-%d_%H-%M-%S"));


    let mut session_options: Vec<String> = Vec::new();
    for (index, session) in sessions_list.iter().enumerate() {
        session_options.push(format!("Session {} ({} packets)", index, session.len()));
    }

    let ans = Select::new("Choose a session:", session_options.clone()).prompt();

    match ans {
        Ok(choice) => {
            let selected_index = choice
                .strip_prefix("Session ")
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse::<usize>().ok());

            if let Some(session_num) = selected_index {
                if let Some(session) = sessions_list.iter().nth(session_num) {
                    for packet in session {
                        save_to_file(packet, filename.as_str());
                    }
                    println!("\nSession saved to: {}", filename);
                } else {
                    println!("Invalid session selection.");
                }
            } else {
                println!("Could not parse selected session.");
            }
        }
        Err(_) => println!("Selection canceled"),
    }
}