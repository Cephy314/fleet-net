# Socket.IO Migration Progress

## Overview
Migrating Fleet-Net from custom TCP protocol to hybrid Socket.IO (control plane) + UDP (audio plane) architecture.

## Key Decisions Made
1. **User Identification**: 16-bit userId (supports 65,536 concurrent users)
2. **Security**: HMAC authentication for UDP packets using per-session secrets
3. **Session Management**: Shared SessionManager between Socket.IO and UDP servers
4. **Architecture**: Socket.IO for control plane, custom UDP for audio plane
5. **Validation**: Created validation module with IP validation using regex patterns

## Completed Tasks
- [x] Installed socket.io and socket.io-client dependencies
- [x] Created Session interface in `/shared/session.ts`
- [x] Updated README.md with technology stack
- [x] Designed message mapping from custom protocol to Socket.IO events
- [x] Created SessionManager class (`/server/session-manager.ts`)
- [x] Created validation module (`/shared/validation.ts`)

## Implementation Plan
1. [x] Create SessionManager class (`/server/session-manager.ts`)
2. [ ] Create Socket.IO server (`/server/network/socket-server.ts`)
3. [ ] Update client to use Socket.IO (`/client/main/network/socket-client.ts`)
4. [ ] Create UDP server for audio (`/server/network/udp-server.ts`)
5. [ ] Update client UDP audio sender with HMAC

## Session Interface
- userId: 16-bit ID for UDP packets
- socketId: Socket.IO connection ID
- udpSecret: Buffer for HMAC verification
- udpAddress/udpPort: Learned from first UDP packet
- permissions: Set of permission strings
- subscribedChannels: Set of channel IDs
- connectedAt: UTC Unix timestamp
- lastUdpActivity: UDP activity timestamp
- clientVersion: Optional client version string

## SessionManager Implementation Details
### Features Implemented
- Singleton pattern for sharing between servers
- Fast O(1) lookups using object properties for UDP packet routing
- Dynamic user ID allocation with wraparound (1-65535, 0 reserved)
- Multiple lookup methods: by userId, socketId, UDP endpoint
- NAT change handling for UDP endpoints
- Full session cleanup on removal
- Input validation for UDP endpoints

### Key Methods
- `createSession(socketId, clientVersion?)`: Creates new session with unique ID
- `updateUdpEndpoint(userId, address, port)`: Updates/sets UDP endpoint
- `removeSession(userId)`: Removes session and cleans up all references
- `getSessionByUserId/SocketId/UdpEndpoint()`: Fast lookups

## Next Step
Create Socket.IO server (`/server/network/socket-server.ts`) to handle:
- Client connections and authentication
- Session creation using SessionManager
- Event handlers for all mapped messages
- Channel management (join/leave)
- User state synchronization
- Permission management
- Heartbeat/keepalive mechanism
- Disconnection cleanup