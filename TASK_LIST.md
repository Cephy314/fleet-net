# Fleet Net TODO

## Current Tasks

### Code Quality Improvements (From Code Review) - COMPLETED
- [x] Fix memory layout in PacketHeader (remove `repr(C)` or use `packed`)
- [x] Optimize string allocations (use `Cow<str>` where appropriate)
    - [x] Error messages in FleetNetError
    - [x] Protocol messages (ControlMessage fields)
    - [x] Session immutable strings (kept as String for simplicity)
    - [x] Role system strings (kept as String - all user-defined)
    - [x] Channel names (kept as String - all user-defined)
- [x] Add missing derive implementations (PartialEq, Eq, Hash)

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
        - [x] Message framing with length prefix
        - [x] JSON serialization/deserialization
        - [ ] Add protocol versioning support (NEXT - needed for HMAC)
        - [ ] Add HMAC validation for message integrity (HIGH PRIORITY - implement now)
        - [ ] Complete security tests (oversized messages, invalid data)
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
- [x] Connection struct moved to protocol crate (shared between client/server)

## Future Enhancements
- [ ] Database integration for persistent storage
- [ ] Rate limiting and DDoS protection
- [ ] Admin dashboard
- [ ] Audit logging system

## Future Optimizations (Post-MVP)
- [ ] Add Permission newtype wrapper for type safety
- [ ] Pre-allocate vectors in packet parsing
- [ ] Profile and optimize hot paths
- [ ] Consider arena allocation for short-lived objects

## Documentation
- [ ] API documentation
- [ ] Architecture diagrams
- [ ] Deployment guide
- [ ] Development setup guide
- [ ] Security best practices guide
