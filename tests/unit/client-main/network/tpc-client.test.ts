import { EventEmitter } from 'node:events';
import * as net from 'node:net';
import { TcpClient } from '../../../../client/main/network/tcp-client';

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

  beforeEach(() => {
    // Create a fresh mock socket for each test
    mockSocket = new MockSocket();

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
      type: 'handshake_ack',
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
    expect(mockSocket.write).toHaveBeenCalledWith(
      `${JSON.stringify({ type: 'handshake', clientVersion: '1.0.0' })} \n`,
    );

    // Simulate server response
    mockSocket.emit('data', `${JSON.stringify(mockHandshakeResponse)}\n`);
  });
});
