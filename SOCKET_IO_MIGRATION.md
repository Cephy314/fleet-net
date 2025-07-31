# Socket.IO Migration Progress

## Overview
Migrating Fleet-Net from custom TCP protocol to hybrid Socket.IO (control plane) + UDP (audio plane) architecture.

## Key Decisions Made
1. **User Identification**: 16-bit userId (supports 65,536 concurrent users)
2. **Security**: HMAC authentication for UDP packets using per-session secrets
3. **Session Management**: Shared SessionManager between Socket.IO and UDP servers
4. **Architecture**: Socket.IO for control plane, custom UDP for audio plane

## Completed Tasks
- [x] Installed socket.io and socket.io-client dependencies
- [x] Created Session interface in `/shared/session.ts`
- [x] Updated README.md with technology stack
- [x] Designed message mapping from custom protocol to Socket.IO events

## Implementation Plan
1. [ ] Create SessionManager class (`/server/session-manager.ts`)
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

## Next Step
Create SessionManager class to handle:
- User ID pool management (16-bit space)
- Session storage and retrieval
- Session cleanup on disconnect
- Singleton pattern for sharing between servers