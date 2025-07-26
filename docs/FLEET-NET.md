# Fleet Net Development Specification

## Executive Summary

Fleet Net is a real-time voice communication system designed specifically for MILSIM gaming communities. The system uses a custom protocol (not WebRTC) with a self-hosted server model, supporting up to 10 simultaneous radios per user with authentic military radio simulation features.

## Core Architecture Decisions

### Deployment Model
- **Self-hosted community servers** - MILSIM groups run their own instances
- **No central infrastructure** - Direct connection via IP:Port
- **Port forwarding** for NAT traversal (no STUN/TURN)

### Technical Foundation
- **Custom protocol** development (not building on existing frameworks)
- **Hybrid TCP/UDP architecture**:
  - TCP: Control channel for state management
  - UDP: Audio data streams
- **Pure SFU (Selective Forwarding Unit)** model - *Changed from initial server-side mixing approach*

### Audio Architecture Evolution
**Initial Plan**: Client records -> sends packets with metadata -> server routes packets -> client decodes packets and processes audio -> client plays audio back with correct pan/volume/effects
**Final Decision**: Pure packet forwarding with client-side mixing
- Server forwards packets to channel subscribers without processing
- Clients decode and mix multiple streams locally
- Eliminates quality loss from re-encoding
- Dramatically reduces server CPU requirements

## Network Protocol Design

### TCP Control Channel Responsibilities
- User authentication and session management
- Channel join/leave operations
- Server state synchronization (user lists, channel members)
- Permission validation
- Real-time configuration updates

### UDP Audio Packet Structure
```
  [0-1]   Channel ID (16 bits)
  [2-3]	  Sending User ID (16 bits)
  [4-5]   Sequence Number (16 bits) - Network byte order
  [6-9]   Relative Timestamp (32 bits)
  [10]    Signal Strength (8 bits)
  [11]    Frame Duration (8 bits)
  [12-13] Audio Data Length (16 bits)
  [14-15] HMAC prefix (16 bits)
  [16+]   Opus Audio Payload (variable)
  ```

**Key Design Decisions**:
- **Network byte order** (big-endian) for protocol compliance
- **Relative timestamps** eliminate clock synchronization needs
- **HMAC authentication** prevents packet spoofing
- **Variable frame size** for network adaptation

### Opus Codec Configuration
**Hardcoded quality tiers** (server-selectable):
- **High**: 48 kbps, 20ms frames, Complexity 10
- **Normal**: 32 kbps, 20ms frames, Complexity 8
- **Low**: 16 kbps, 20ms frames, Complexity 6

*Changed from initial tri-casting approach to single server-configured quality*

## Client Architecture

### Technology Stack
- **Electron** framework for cross-platform compatibility
- **Node.js native audio** (PortAudio) in main process
- **Pure UI** in renderer process (no audio handling)
- **Worker threads** for audio processing

### Audio Input Pipeline
1. **Web-Audio**: Use the render thread with worklets to capture and playback audio.
2. **150ms circular pre-buffer** (prevents word clipping)
3. **200ms post-buffer** on PTT release
4. **Global input buffer** feeding all radios (not per-radio buffers)

### PTT (Push-To-Talk) System
- **Multi-input support**: Keyboard, gamepad, Stream Deck
- **Exclusive locking**: First PTT blocks others until release
- **One-deep queue**: Second PTT queues if pressed during active transmission
- **Emergency release**: Global key to unstick any PTT
- **Per-radio bindings**: Independent PTT keys for each of 10 radios

### Client-Side Audio Processing
1. **Adaptive jitter buffer** per incoming stream
2. **Decode Opus** to PCM
3. **Linear mixing** with clipping protection: `(s1 + s2 + ... sN) / sqrt(N)`
4. **Radio DSP effects** (simplified static filters):
   - Bandpass filtering per radio type
   - Noise injection based on signal strength
   - Distortion characteristics
5. **Local playback** of radio sound effects (squelch)

### Jitter Buffer Implementation
- **Adaptive delay**: 20-100ms based on network statistics
- **4σ + safety margin** calculation for target delay
- **Growth rate**: +10ms on underrun
- **Shrink rate**: -1ms per 500ms clean playback
- **Packet loss concealment**: Opus FEC or comfort noise

## Server Architecture

### Technology Stack
- **Node.js** with native C++ addons for performance-critical operations
- **Worker thread pool** for parallel processing
- **SQLite** for permission storage and configuration

### Pure Forwarding Architecture
```javascript
onUDPPacket(packet, sender) {
  channelId = extractChannelId(packet);
  if (!canTransmit(sender, channelId)) return;
  
  subscribers = channelSubscriptions.get(channelId);
  subscribers.forEach(session => {
    if (session !== sender) {
      forwardPacket(packet, session);
    }
  });
}
```

### Performance Optimizations
- **Pre-computed permission cache** (bitset per user)
- **No audio decoding/encoding** on server
- **Direct packet forwarding** (~1μs per operation)
- **Worker threads** only for initial connection handling

## Authentication & Security

### Discord OAuth Integration
- **Login flow**: Discord OAuth2 with `guilds.members.read` scope
- **Guild-specific**: Server specifies required Discord guild ID
- **JWT tokens** with 1-hour expiration and refresh

### Permission System
- **Two-tier role mapping**:
  1. Local roles defined in server
  2. Discord roles mapped to local roles
- **SQLite schema** for persistence
- **Real-time updates** without reconnection required
- **Server-side authority** for all permission checks

### Encryption
- **Always-on encryption** for all client-server communication
- **TLS 1.3** for TCP control channel
- **DTLS** for UDP audio streams
- **Per-session HMAC keys** derived from TLS handshake

## Configuration & Administration

### Server Configuration
- **Local JSON file** for saved servers (client-side)
- **SQLite database** for server-side configuration
- **Owner authentication** via Discord ID in config file
- **In-client administration** tools (no web interface)

### Admin Features
- Real-time role and permission management
- Channel creation and configuration
- User monitoring and moderation tools
- Immediate effect for all configuration changes

## Radio Simulation Features

### Radio Types
1. **Short-range** (Squad/Tactical)
   - Bandpass: 300-3400 Hz
   - Higher noise floor
   - Soft clipping

2. **Ship-wide** (Intercom)
   - Bandpass: 80-8000 Hz
   - Clean audio quality
   - No clipping

3. **Long-range** (Command)
   - Bandpass: 500-2500 Hz
   - Atmospheric noise
   - Hard clipping and distortion

### Transmission Behavior
- **Half-duplex enforcement** per radio
- **No voice activation** - PTT required for all transmissions
- **User-adjustable squelch** controls
- **Local sound effects** synchronized with jitter buffer

## State Synchronization

### Event-Driven Updates (TCP)
- User join/leave events
- Channel subscription changes
- Configuration updates
- *Not used for PTT state*

### PTT State Inference
- Derived from UDP packet flow
- 500ms timeout for transmission end detection
- Triggers UI updates and sound effects

## Implementation Priorities

### MVP Features
1. Basic multi-radio PTT functionality
2. Channel subscriptions and forwarding
3. Discord authentication
4. Simple DSP effects
5. Jitter buffer with adaptive delay

### Future Enhancements
- Signal strength calculation based on game position
- Advanced DSP modeling
- Voice activation detection
- Radio frequency simulation
- Environmental audio effects

## APENDIX
### A: Requirements
* User must be able to manage server "Bookmarks" for common connections.
* User can select which audio devices they are using for input and output.
* User can perform audio tests to ensure hardware is working like sound playback and loop-back for recording tests.
* User can enable or disable Automatic Gain Control for their input.
* User can enable or disable Echo Cancellation for their input.
* User can enable or disable Noise Reduction for their input.
* User can control overall input volume / sensitivity
* User can control overall output volume.
* User can join a voice channel.
* User can leave a voice channel.
* User can deafen and mute themselves.
* User can mute other users locally.
* User can control other users volume locally.
* User can create a Radio in the Radio Rack.
* User can tune the Radio to frequencies.
* User can control the volume of the radio.
* User can control the pan of the Radio.
* User can temporarily DIM the radio (lower volume to 10%)
* User can mute or power off the radio.
* User can make many radios.
* User can set Keybinds to control radio volume
* User can set keybinds to speak on a radio.
* User can set keybinds to control radio pan
* user can set keybinds to DIM a radio.
* user can set keybinds to mute a radio.
* user can set keybinds to speak in the active voice channel.
* User can see other users in their and other channels.
* User can collapse the categories and channels on the channel tree.
* User can set them selves AWAY which will deafen and mute them.
* User can disconnect from the server.
* User can switch from one channel to another
* Users with appropriate permissions can move other users from one channel to another.
* Users with appropriate permissions can globally mute other users.
* Users with appropriate permissions can kick other users.
* Users with appropriate permissions can ban other users.
* Users with appropriate permissions can Create channels.
* Users with appropriate permissions can Edit channels.
* Users with appropriate permissions can Delete channels
* Users with appropriate permissions can Create categories.
* Users with appropriate permissions can Delete categories.
* Users with appropriate permissions can Move categories.
* Users with appropriate permissions can Move channels.