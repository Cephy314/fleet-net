import {
  type HandshakeAckMessage,
  type HandshakeMessage,
  MessageType,
} from '../../../../shared/protocol/messages';
import { ProtocolHandler } from '../../../../shared/protocol/protocol-handler';

describe('ProtocolHandler', () => {
  let handler: ProtocolHandler;

  beforeEach(() => {
    handler = new ProtocolHandler();
  });

  it('should encode and decode a handshake message', () => {
    const message: HandshakeMessage = {
      type: MessageType.HANDSHAKE,
      clientVersion: '1.0.0',
    };

    const encoded = handler.encode(message);
    expect(encoded).toBeInstanceOf(Buffer);

    // addData handles the length prefix, so we can decode directly
    const messages = handler.addData(encoded);
    expect(messages).toHaveLength(1);
    expect(messages[0]).toEqual(message);
  });

  it('should encode and decode a handshake ack message', () => {
    const message: HandshakeAckMessage = {
      type: MessageType.HANDSHAKE_ACK,
      sessionId: 'test-session-123',
      serverVersion: '1.0.0',
    };

    const encoded = handler.encode(message);
    expect(encoded).toBeInstanceOf(Buffer);
    const messages = handler.addData(encoded);
    expect(messages).toHaveLength(1);
    expect(messages[0]).toEqual(message);
  });

  it('should handle incomplete messages correctly', () => {
    const message: HandshakeMessage = {
      type: MessageType.HANDSHAKE,
      clientVersion: '1.0.0',
    };

    const encoded = handler.encode(message);
    console.log('Full message length:', encoded.length);
    console.log('Full message buffer:', encoded);

    // Try to decode only half the message
    const halfMessage = encoded.slice(0, Math.floor(encoded.length / 2));
    console.log('Half message length:', halfMessage.length);

    // This should not decode anything or throw an error
    const messages = handler.addData(halfMessage);
    console.log('Decoded from half:', messages);

    // Now add the rest
    const restMessage = encoded.slice(Math.floor(encoded.length / 2));
    const messages2 = handler.addData(restMessage);
    console.log('Decoded after adding rest:', messages2);
  });

  it('should handle streaming data with multiple messages', () => {
    const message1: HandshakeMessage = {
      type: MessageType.HANDSHAKE,
      clientVersion: '1.0.0',
    };
    const message2: HandshakeAckMessage = {
      type: MessageType.HANDSHAKE_ACK,
      sessionId: 'session-456',
      serverVersion: '1.0.0',
    };

    // Encode both messages (now with length prefix)
    const encoded1 = handler.encode(message1);
    const encoded2 = handler.encode(message2);

    // Test partial message handling
    // Split first message in half (including length prefix)
    const halfPoint = Math.floor(encoded1.length / 2);
    const chunk1 = encoded1.slice(0, halfPoint);
    const chunk2 = Buffer.concat([encoded1.slice(halfPoint), encoded2]);

    // First chunk - incomplete message (has length but not full data)
    let messages = handler.addData(chunk1);
    expect(messages).toHaveLength(0);

    // Second chunk - completes first message and adds second
    messages = handler.addData(chunk2);
    expect(messages).toHaveLength(2);
    expect(messages[0]).toEqual(message1);
    expect(messages[1]).toEqual(message2);

    // Test very small chunks (less than length header)
    handler.reset();
    messages = handler.addData(encoded1.slice(0, 2)); // Only 2 bytes of length
    expect(messages).toHaveLength(0);

    messages = handler.addData(encoded1.slice(2)); // Rest of message
    expect(messages).toHaveLength(1);
    expect(messages[0]).toEqual(message1);
  });
});
