export const MessageType = {
  HANDSHAKE: 'handshake',
  HANDSHAKE_ACK: 'handshake_ack',
  ERROR: 'error',
  USER_JOIN: 'user_join',
  USER_LEAVE: 'user_leave',
  CHANNEL_JOIN: 'channel_join',
  CHANNEL_LEAVE: 'channel_leave',
} as const;

export type MessageType = (typeof MessageType)[keyof typeof MessageType];

// Message Interfaces
export interface HandshakeMessage {
  type: typeof MessageType.HANDSHAKE;
  clientVersion: string;
}

export interface HandshakeAckMessage {
  type: typeof MessageType.HANDSHAKE_ACK;
  sessionId: string;
  serverVersion: string;
}

export interface ErrorMessage {
  type: typeof MessageType.ERROR;
  code: string;
  message: string;
}

// Union type of all messages
export type Message = HandshakeMessage | HandshakeAckMessage | ErrorMessage;
