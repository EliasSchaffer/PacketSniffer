# PacketSniffer

> **⚠️ Disclaimer:** This README was AI-generated using GitHub Copilot.

A lightweight **network packet sniffer** written in Rust that captures and displays network traffic in real-time. Built as a learning project to understand Rust systems programming, network protocols, and packet parsing.

## Features

✨ **Core Capabilities:**
- 🔍 Real-time packet capture from any network interface
- 📊 Support for **IPv4 and IPv6** address parsing
- 🔌 **TCP and UDP** transport layer protocol detection
- 💾 Packet storage in a linked list (for future processing)
- 📝 Payload display in both **HEX and ASCII** formats
- 🎯 Interactive network interface selection via CLI

## Requirements

- **Rust 1.56+** (edition 2021)
- **Windows/Linux/macOS** with libpcap installed
  - **Windows:** WinPcap or Npcap
  - **Linux:** `sudo apt-get install libpcap-dev`
  - **macOS:** Installed by default

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd PacketSniffer
```

2. Build the project:
```bash
cargo build --release
```

3. Run with administrative privileges (required for packet capture):
```bash
# Windows
cargo run --release

# Linux/macOS
sudo cargo run --release
```

## Usage

1. Start the application
2. Select a network interface from the interactive menu
3. The program will display captured packets in real-time
4. Each packet shows:
   - Source/Destination IPv4 and IPv6 addresses
   - Protocol type (TCP/UDP)
   - Source/Destination ports
   - Packet payload in HEX and ASCII

**Example Output:**
```
_______________________________
IPv4 Source: 192.168.1.100
IPv4 Destination: 8.8.8.8
IPv6 Source: ::
IPv6 Destination: ::
Protocol: TCP
Source Port: 54321
Destination Port: 443
Payload HEX: 47 45 54 20 2f | ASCII: GET /
_______________________________
```

## Project Structure

```
PacketSniffer/
├── Cargo.toml                 # Project manifest
├── README.md                  # This file
└── src/
    ├── main.rs               # Entry point, packet capture loop
    ├── ip_address.rs         # IPv4/IPv6 address handling
    ├── protocol.rs           # Transport protocol (TCP/UDP) info
    ├── parsed_packet.rs      # Packet data structure
    └── parsed_packet.rs      # Packet display logic
```

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `pcap` | 2.4.0 | Network packet capture |
| `inquire` | 0.9.4 | Interactive CLI prompts |
| `etherparse` | 0.15 | Network packet parsing |
| `string-builder` | 0.2 | String formatting utilities |

## Known Issues & TODO

### Current Limitations:
- ⚠️ Minimal error handling (uses `.unwrap()` in critical paths)
- ⚠️ No persistent storage of captured packets
- ⚠️ Limited to TCP/UDP protocols (other transports ignored)
- ⚠️ Ports stored as strings (should be `u16`)

### Planned Improvements:
- 🔧 Robust error handling with `Result<T, E>`
- 🎨 TUI dashboard with real-time statistics (using crossterm)
- 🔗 Protocol filtering (by port, IP, protocol type)
- 💾 Export to PCAP file format
- 📈 Packet statistics and counters
- 🧪 Unit tests for parsing functions
- 📚 Comprehensive documentation

## License

[Specify your license here - MIT, Apache 2.0, etc.]

## Author

Created as a first Rust project | February 2026

---

**Learning Resources Used:**
- [The Rust Book](https://doc.rust-lang.org/book/)
- [etherparse documentation](https://docs.rs/etherparse/)
- [pcap crate](https://docs.rs/pcap/)

---

*This is an educational project. Use responsibly and ensure you have permission to monitor network traffic on your systems.*
