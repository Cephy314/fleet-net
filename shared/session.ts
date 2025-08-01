export interface Session {
    // Identity
    userId: number; // 16-bit ID for UDP packets
    socketId: string; // Socket.IO connection ID
    discordId?: string; // From OAuth

    // Security
    udpSecret: Buffer; // For HMAC verification

    // Network
    udpAddress?: string; // Client's UDP endpoint
    udpPort?: number; // Learned from first UDP packet
    lastUdpActivity?: number; // Unix timestamp

    // State
    permissions: Set<string>;
    subscribedChannels: Set<number>;
    muted?: boolean; // Mute state for audio
    deafened?: boolean; // Deafen state for audio

    // Metadata
    connectedAt: number; // Unix timestamp
    clientVersion?: string;
}
