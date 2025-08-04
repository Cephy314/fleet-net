# Fleet Net TODO

## Current Tasks

### Common Module
- [x] Add Session helper methods in `session.rs`
    - [x] `update_activity()` - Update last_active timestamp
    - [x] `is_idle(timeout)` - Check if session is idle

- [x] Create radio subscription structures (moved to client crate)
    - [x] Radio struct for individual radio configuration
    - [x] RadioSubscription for user's radio state (server-side routing approach)
    - [x] Support for up to 10 radios per user
    - [x] Radio tuning to channels/frequencies
    - [x] Volume and pan control per radio
    - [x] PTT keybinding configuration (radio ID-based)
    - [x] Radio mute/dim states

### Protocol Module
- [x] Implement TCP message serialization/deserialization
- [x] Implement UDP packet parsing/building
- [x] Add HMAC validation structure for packets

### Next Steps (After Current Tasks)
- [ ] Add comprehensive test coverage for common crate
    - [ ] Test user/role/permission structures
    - [ ] Test session helper methods
    - [ ] Test channel permission overrides
    - [ ] Test error handling edge cases
    
- [ ] Start server implementation
    - [ ] TCP control channel handler
    - [ ] UDP packet forwarding
    - [ ] Session management
    - [ ] Channel subscription handling

- [ ] Start client implementation
    - [ ] Tauri integration
    - [ ] Audio input/output handling
    - [ ] PTT system
    - [ ] Jitter buffer

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
