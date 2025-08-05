# Fleet Net TODO

## Current Tasks

### Code Quality Improvements (From Code Review)
- [x] Fix memory layout in PacketHeader (remove `repr(C)` or use `packed`)
- [x] Optimize string allocations (use `Cow<str>` where appropriate)
    - [x] Error messages in FleetNetError
    - [x] Protocol messages (ControlMessage fields)
    - [x] Session immutable strings (kept as String for simplicity)
    - [x] Role system strings (kept as String - all user-defined)
    - [x] Channel names (kept as String - all user-defined)
- [ ] Add Permission newtype wrapper for type safety
- [x] Add missing derive implementations (PartialEq, Eq, Hash)
- [ ] Pre-allocate vectors in packet parsing

### Test Coverage (Partially Complete)
- [x] Basic test structure added for common crate modules
- [ ] Expand test coverage for edge cases
    - [ ] Test error handling paths
    - [ ] Test permission priority resolution edge cases
    - [ ] Add property-based testing for packet serialization
    - [ ] Test session timeout behaviors

### Server Implementation (Started)
- [x] Basic server setup with logging
- [ ] TCP control channel implementation
    - [ ] Connection handling
    - [ ] Message routing
    - [ ] Authentication flow
- [ ] UDP audio packet forwarding
    - [ ] Packet validation with HMAC
    - [ ] Channel-based routing
    - [ ] Jitter buffer implementation
- [ ] Session management
    - [ ] Session creation/destruction
    - [ ] Idle timeout handling
    - [ ] Reconnection support
- [ ] Channel subscription system
    - [ ] Join/leave handling
    - [ ] Permission enforcement
    - [ ] State synchronization

### Client Implementation (Started)
- [x] Basic Tauri application setup
- [x] Radio structures defined
- [ ] Core client functionality
    - [ ] Server connection management
    - [ ] Authentication UI
    - [ ] Reconnection logic
- [ ] Audio system
    - [ ] Audio device enumeration
    - [ ] Input/output handling
    - [ ] Opus encoding/decoding
    - [ ] Radio effect processing (HF, UHF, VHF effects)
- [ ] PTT (Push-to-Talk) system
    - [ ] Keybinding configuration UI
    - [ ] Multi-radio PTT support
    - [ ] PTT state management
- [ ] Client-side jitter buffer
- [ ] UI Implementation
    - [ ] Channel list view
    - [ ] Radio control panel
    - [ ] User list with states
    - [ ] Settings/configuration

## Completed Tasks
- [x] User structure with Discord integration
- [x] Session structure
- [x] Permission system with bitflags
- [x] Role mapping structure
- [x] User audio state tracking
- [x] Channel structure with permission overrides
- [x] Refactored into separate modules
- [x] TCP message serialization/deserialization with ControlMessage enum
- [x] UDP packet parsing/building with AudioPacket and PacketHeader
- [x] HMAC validation structure (prefix field in PacketHeader)
- [x] Comprehensive error handling system (PacketError integration)
- [x] Full test coverage for message and packet serialization
- [x] Session helper methods (update_activity, is_idle)
- [x] Radio system design and structures (moved to client crate)
- [x] Common module with session helper methods
- [x] Protocol module with TCP/UDP implementations

## Future Enhancements
- [ ] Database integration for persistent storage
- [ ] Redis integration for session caching
- [ ] Kubernetes deployment configuration
- [ ] Monitoring and metrics (Prometheus)
- [ ] Rate limiting and DDoS protection
- [ ] WebRTC support for browser clients
- [ ] Admin dashboard
- [ ] Audit logging system

## Documentation
- [ ] API documentation
- [ ] Architecture diagrams
- [ ] Deployment guide
- [ ] Development setup guide
- [ ] Security best practices guide
