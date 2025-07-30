import { TcpClient } from '../../../client/main/network/tcp-client';
import { TcpServer } from '../../../server/network/tcp-server';

describe('TCP Client-Server Integration', () => {
  let server: TcpServer;
  let client: TcpClient;
  const testPort = 3456;

  beforeEach(async () => {
    server = new TcpServer();
    client = new TcpClient();
    await server.start(testPort);
  });

  afterEach(async () => {
    client.disconnect();
    await server.stop();
  });

  it('should connect and perform handshake', (done) => {
    client.on('handshake_complete', (response) => {
      expect(response.sessionId).toBeDefined();
      expect(response.serverVersion).toBe('1.0.0');
      expect(client.getSessionId()).toBe(response.sessionId);
    });

    client.connect('localhost', testPort);
  });
});
