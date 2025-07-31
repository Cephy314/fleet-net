# Fleet Net

<div align="center">

![Node.js](https://img.shields.io/badge/Node.js-339933?style=for-the-badge&logo=nodedotjs&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)
![Electron](https://img.shields.io/badge/Electron-47848F?style=for-the-badge&logo=electron&logoColor=white)
![Socket.io](https://img.shields.io/badge/Socket.io-010101?style=for-the-badge&logo=socketdotio&logoColor=white)
![SQLite](https://img.shields.io/badge/SQLite-003B57?style=for-the-badge&logo=sqlite&logoColor=white)

[![License](https://img.shields.io/badge/license-AGPL--3.0-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-In%20Development-yellow)](https://github.com/yourusername/fleet-net)
[![Discord](https://img.shields.io/badge/Discord-OAuth2-5865F2?logo=discord&logoColor=white)](docs/FLEET-NET.md#authentication--security)

</div>

## 🎮 Overview

Fleet Net is a **real-time voice communication system** designed specifically for **MILSIM gaming communities**. It provides authentic military radio simulation with support for up to 10 simultaneous radios per user, featuring realistic audio effects and half-duplex communication.

### 🔑 Key Features

- **🎙️ Multi-Radio Support**: Up to 10 simultaneous radios per user
- **🔊 Realistic Radio Effects**: Bandpass filtering, noise injection, and distortion
- **🏠 Self-Hosted**: Community-owned servers with no central infrastructure
- **🔒 Secure Communication**: TLS 1.3 for control, DTLS for audio
- **🎮 Gaming Integration**: Multiple PTT inputs (keyboard, gamepad, Stream Deck)
- **📡 Low Latency**: Pure SFU architecture with direct packet forwarding

## 🏗️ Architecture

### Technology Stack
- **Client**: Electron + TypeScript + Web Audio API
- **Server**: Node.js + TypeScript + Worker Threads
- **Control Plane**: Socket.IO (TCP)
- **Audio Plane**: Custom UDP protocol with HMAC authentication
- **Database**: SQLite for permissions and configuration
- **Audio Codec**: Opus (configurable quality tiers)

### Network Protocol
```
Control Channel (Socket.IO/TCP):
- User authentication
- Channel management
- State synchronization
- Permission updates

Audio Channel (UDP):
- 16-bit Channel ID
- 16-bit User ID
- HMAC authentication
- Opus audio payload
```

## 🚧 Current Status

This project is actively under development, migrating from a custom TCP protocol to a hybrid Socket.IO/UDP architecture.

### Completed ✅
- Core architecture design
- Protocol specifications
- Session management interface
- Technology stack selection

### In Progress 🔄
- Socket.IO server implementation
- UDP audio server
- Client migration to Socket.IO
- Session manager implementation

### Upcoming 📋
- Discord OAuth2 integration
- Audio DSP pipeline
- Radio simulation effects
- Permission system
- Admin tools

## 🚀 Getting Started

> ⚠️ **Note**: Fleet Net is in early development. Setup instructions will be provided once the MVP is functional.

### Prerequisites
- Node.js 18+
- npm or yarn
- Discord application (for OAuth2)

### Installation
```bash
# Clone the repository
git clone https://github.com/yourusername/fleet-net.git
cd fleet-net

# Install dependencies
npm install

# Build the project
npm run build
```

## 📖 Documentation

- [**Fleet Net Development Specification**](docs/FLEET-NET.md) - Complete technical specification
- [**Socket.IO Migration Progress**](SOCKET_IO_MIGRATION.md) - Current migration status

## 🎯 Minimum Viable Product (MVP)

The MVP will include:
1. Basic multi-radio PTT functionality
2. Channel subscriptions and forwarding
3. Discord authentication
4. Simple DSP effects
5. Jitter buffer with adaptive delay

## 🤝 Contributing

Fleet Net is currently in early development. Contribution guidelines will be established once the core functionality is implemented.

## 📄 License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0) - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Project Specification](docs/FLEET-NET.md)
- [Discord OAuth2 Setup Guide](#) (Coming Soon)
- [Server Administration Guide](#) (Coming Soon)
