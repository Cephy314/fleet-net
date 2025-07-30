import { EventEmitter } from 'node:events';
import * as net from 'node:net';
import { TcpClient } from '../../../../client/main/network/tcp-client';
import { MessageType } from '../../../../shared/protocol/messages';
import { ProtocolHandler } from '../../../../shared/protocol/protocol-handler';

jest.mock('net');

class MockSocket extends EventEmitter {
  connect = jest.fn();
  write = jest.fn();
  end = jest.fn();
  destroy = jest.fn();
  setEncoding = jest.fn();
}

describe('TcpClient', () => {
  let client: TcpClient;
  let mockSocket: MockSocket;
  let protocolHandler: ProtocolHandler;

  beforeEach(() => {
    // Create a fresh mock socket for each test
    mockSocket = new MockSocket();
    protocolHandler = new ProtocolHandler();

    // Mock createConnection to return our mock socket
    (net.createConnection as jest.Mock).mockReturnValue(mockSocket);

    client = new TcpClient();
  });

  afterEach(() => {
    jest.clearAllMocks();
  });

  it('should connect to a TCP server', (done) => {
    client.on('connected', () => {
      expect(client.isConnected()).toBe(true);
      done();
    });

    client.connect('localhost', 3000);

    // Simulate successful connection
    mockSocket.emit('connect');
  });

  it('should emit error when connection fails', (done) => {
    client.on('error', (error) => {
      expect(error.message).toBe('Connection refused');
      expect(client.isConnected()).toBe(false);
      done();
    });
    client.connect('localhost', 3000);
    mockSocket.emit('error', new Error('Connection refused'));
  });

  it('should perform handshake after connection', (done) => {
    const mockHandshakeResponse = {
      type: MessageType.HANDSHAKE_ACK,
      sessionId: 'test-session-123',
      serverVersion: '1.0.0',
    };

    client.on('handshake_complete', (response) => {
      expect(response.sessionId).toBe('test-session-123');
      expect(response.serverVersion).toBe('1.0.0');
      expect(client.getSessionId()).toBe('test-session-123');
      done();
    });

    client.connect('localhost', 3000);
    mockSocket.emit('connect');

    // Verify handshake message was sent
    expect(mockSocket.write).toHaveBeenCalled();
    const sentData = mockSocket.write.mock.calls[0][0];
    expect(sentData).toBeInstanceOf(Buffer);

    // Decode what was sent to verify it's correct.
    const testHandler = new ProtocolHandler();
    const sentMessages = testHandler.addData(sentData);
    expect(sentMessages).toHaveLength(1);
    expect(sentMessages[0]).toEqual({
      type: MessageType.HANDSHAKE,
      clientVersion: '1.0.0',
    });

    // Simulate server response
    const responseBuffer = protocolHandler.encode(mockHandshakeResponse);
    mockSocket.emit('data', responseBuffer);
  });
});
