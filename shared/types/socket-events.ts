// Generic response type for all Socket.IO callbacks.
export interface SocketResponse<T = void> {
    success: boolean;
    error?: string;
    data?: T;
}

// Event-specific response data types.

export interface ChannelEventResponse {
    channelId: number;
}

export interface UserStateResponse {
    muted?: boolean;
    deafened?: boolean;
}

export interface UserConnectionResponse {
    userId: number;
    channelId: number;
}
