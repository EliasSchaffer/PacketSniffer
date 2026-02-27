# PacketSniffer

A lightweight Rust CLI packet sniffer for learning network capture, parsing, and session handling.

## Features

- Real-time capture from a selected network interface
- IPv4/IPv6 source and destination parsing
- TCP/UDP detection with source and destination ports
- Payload output in HEX + ASCII
- Session workflow via interactive menu:
  - Start new session
  - Show captured sessions
  - Save a selected session to file
- Stop live capture with **SPACE**

## Requirements

- Rust (edition 2024)
- Windows/Linux/macOS with libpcap support
  - Windows: Npcap or WinPcap
  - Linux: `sudo apt-get install libpcap-dev`
  - macOS: libpcap is usually available by default
- Run with elevated privileges (required for packet capture)

## Installation

```bash
git clone <repository-url>
cd PacketSniffer
cargo build --release
```

## Run

```bash
# Windows
cargo run --release

# Linux/macOS
sudo cargo run --release
```

## Usage

1. Choose a network interface.
2. In the menu, select **Start new Session**.
3. Capture runs until you press **SPACE**.
4. Use **Show Sessions** to list captured sessions.
5. Use **Save Sessions** to pick one session and write it to a timestamped file.

Saved files currently use this format:

`SessionYYYY-MM-DD_HH-MM-SS.txt`

## Example Packet Output

```text
_______________________________
IPv4 Source: 192.168.1.100 | IPv4 Destination: 8.8.8.8 | IPv6 Source: :: | IPv6 Destination: ::
Protocol: TCP | Source Port: 54321 | Destination Port: 443
Payload HEX: 47 45 54 20 2f  | ASCII: GET /
_______________________________
```

## Project Structure

```text
PacketSniffer/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    ├── ip_address.rs
    ├── protocol.rs
    ├── payload.rs
    └── parsed_packet.rs
```

## Dependencies

| Crate | Version |
|---|---|
| `pcap` | `2.4.0` |
| `inquire` | `0.9.4` |
| `etherparse` | `0.15` |
| `chrono` | `0.4` |
| `crossterm` | `0.28` |

---

Educational project. Use responsibly and only on networks/systems where you have permission to capture traffic.
