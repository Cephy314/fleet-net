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
Fleet Net is built on a **hybrid TCP/UDP architecture** with a custom protocol that supports real-time voice communication. The system uses a **Selective Forwarding Unit (SFU)** model, where the server forwards audio packets to clients without processing them, allowing for efficient client-side mixing.

## 🚧 Current Status

This project is actively under development
## 🤝 Contributing

Fleet Net is currently in early development. Contribution guidelines will be established once the core functionality is implemented.

## 📄 License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0) - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Project Specification](docs/FLEET-NET.md)
- [Discord OAuth2 Setup Guide](#) (Coming Soon)
- [Server Administration Guide](#) (Coming Soon)
