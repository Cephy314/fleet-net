# Fleet Net

  <div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=for-the-badge&logo=tauri&logoColor=black)
![Tokio](https://img.shields.io/badge/Tokio-00ADD8?style=for-the-badge&logo=rust&logoColor=white)
![SQLite](https://img.shields.io/badge/SQLite-003B57?style=for-the-badge&logo=sqlite&logoColor=white)

[![License](https://img.shields.io/badge/license-AGPL--3.0-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-In%20Development-yellow)](https://github.com/yourusername/fleet-net)
[![Discord](https://img.shields.io/badge/Discord-OAuth2-5865F2?logo=discord&logoColor=white)](docs/FLEET-NET.md#authentication--security)

  </div>

## 🎮 Overview

Fleet Net is a **real-time voice communication system** designed specifically for **MILSIM gaming communities**. It provides authentic military
radio simulation with support for up to 10 simultaneous radios per user, featuring realistic audio effects and half-duplex communication.

### 🔑 Key Features

- **🎙 Multi-Radio Support**: Up to 10 simultaneous radios per user
- **🔊 Realistic Radio Effects**: Bandpass filtering, noise injection, and distortion
- **🏠 Self-Hosted**: Community-owned servers with no central infrastructure
- **🔒 Secure Communication**: TLS 1.3 for control, DTLS for audio
- **🎮 Gaming Integration**: Multiple PTT inputs (keyboard, gamepad, Stream Deck)
- **📡 Low Latency**: Pure SFU architecture with direct packet forwarding

## 🏗 Architecture

Fleet Net is built on a **hybrid TCP/UDP architecture** with a custom protocol:
- **TCP**: Control channel for state management and authentication
- **UDP**: Audio data streams with low-latency packet forwarding
- **Pure SFU Model**: Server forwards packets without processing, clients handle mixing

## 🛠 Tech Stack

- **Language**: Rust
- **Client Framework**: Tauri (cross-platform desktop)
- **Server**: Tokio-based async runtime
- **Audio**: Opus codec with custom DSP effects
- **Database**: SQLite for permissions and configuration
- **Authentication**: Discord OAuth2 with JWT tokens

## 📁 Project Structure

```plaintext
fleet-net/
├── crates/
│   ├── fleet-net-client/    # Tauri desktop application
│   ├── fleet-net-server/    # TCP/UDP server
│   ├── fleet-net-protocol/  # Network protocol definitions
│   ├── fleet-net-audio/     # Audio processing and codecs
│   └── fleet-net-common/    # Shared utilities and types
├── docs/                    # Documentation
├── scripts/                 # Build and development scripts
└── Cargo.toml              # Workspace configuration
```
## 🚀 Getting Started

### Prerequisites

- Rust 1.88+ (stable)
- System dependencies:
  ```bash
  # Ubuntu/Debian
  sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev cmake libasound2-dev

  # Fedora
  sudo dnf install webkit2gtk4.1-devel openssl-devel gtk3-devel libappindicator-gtk3-devel librsvg2-devel cmake alsa-lib-devel

  # Arch
  sudo pacman -S webkit2gtk-4.1 base-devel curl wget openssl gtk3 libappindicator-gtk3 librsvg cmake alsa-lib

### Building

#### Clone the repository
`git clone https://github.com/yourusername/fleet-net.git`  
`cd fleet-net`

#### Build all components
`cargo build`

#### Run the server
`cargo run -p fleet-net-server`

#### Run the client (in another terminal)
`cargo run -p fleet-net-client`

### Development

#### Format code
`make fmt`

#### Run lints
`make lint`

#### Run tests
`make test`

### Install pre-commit hooks
`make install-hooks`

## 🚧 Current Status

This project is actively under development. Core features being implemented:
- Basic multi-radio PTT functionality
- Channel subscriptions and forwarding
- Discord authentication
- Audio DSP effects
- Jitter buffer implementation

## 🤝 Contributing

Fleet Net is currently in early development. Contribution guidelines will be established once the core functionality is implemented.

## 📄 License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0) - see the LICENSE file for details.

## 🔗 Links

- [docs/FLEET-NET.md](docs/FLEET-NET.md) - Detailed documentation
