# Fleet Net TODO

## Current Tasks

### Common Module
- [ ] Add Session helper methods in `session.rs`
    - [ ] `can_transmit(channel_id)` - Check if user can speak on a channel
    - [ ] `can_receive(channel_id)` - Check if user can listen to a channel
    - [ ] `update_activity()` - Update last_active timestamp
    - [ ] `is_idle(timeout)` - Check if session is idle

- [ ] Create radio subscription structures
    - [ ] Radio struct for individual radio configuration
    - [ ] RadioSubscription for user's radio state
    - [ ] Support for up to 10 radios per user
    - [ ] Radio tuning to channels/frequencies
    - [ ] Volume and pan control per radio
    - [ ] PTT keybinding configuration
    - [ ] Radio mute/dim states

### Protocol Module
- [ ] Implement TCP message serialization/deserialization
- [ ] Implement UDP packet parsing/building
- [ ] Add HMAC validation for packets

### Next Steps (After Current Tasks)
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
