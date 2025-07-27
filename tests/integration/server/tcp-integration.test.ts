import { Socket } from 'node:net';
import { TcpServer } from '../../../server/network/tcp-server';

describe('TCP Server Integration', () => {
  let server: TcpServer;

  beforeEach(() => {
    server = new TcpServer();
  });
  afterEach(async () => {
    if (server.isListening()) {
      await server.stop();
    }
  });

  it('should handle multiple clients connecting and disconnecting', async () => {
    await server.start(4000);

    const connections: Array<{ socket: Socket; sessionId: string }> = [];

    // Track connections
    server.on('connection', (socket: Socket, sessionId: string) => {
      connections.push({ socket, sessionId });
    });

    // Connect 5 clients
    const clients: Socket[] = [];
    for (let i = 0; i < 5; i++) {
      const client = new Socket();
      clients.push(client);
      await new Promise<void>((resolve) => {
        client.connect(4000, 'localhost', resolve);
      });
    }

    // Verify all connected
    expect(server.getClientCount()).toBe(5);
    expect(connections).toHaveLength(5);

    // Verify all session IDs are unique
    const sessionIds = new Set(connections.map((c) => c.sessionId));
    expect(new Set(sessionIds).size).toBe(5);

    // Disconnect some clients
    clients[0].end();
    clients[2].end();
    await new Promise((resolve) => setTimeout(resolve, 100)); // Wait for disconnections

    expect(server.getClientCount()).toBe(3);

    // Clean up remaining clients
    clients[1].end();
    clients[3].end();
    clients[4].end();
    await new Promise((resolve) => setTimeout(resolve, 100)); // Wait for all disconnections

    expect(server.getClientCount()).toBe(0);
  });
});
