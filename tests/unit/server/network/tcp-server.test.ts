import { TcpServer } from '../../../../server/network/tcp-server';

describe('TcpServer', () => {
  let server: TcpServer;
  beforeEach(() => {
    server = new TcpServer();
  });
  afterEach(async () => {
    if (server.isListening()) {
      await server.stop();
    }
  });
  describe('start', () => {
    it('should start listening on the specified port', async () => {
      const port = 3100;

      await server.start(port);

      expect(server.isListening()).toBe(true);
      expect(server.getPort()).toBe(port);
    });

    it('should accept TCP connections', async () => {
      const port = 3101;

      await server.start(port);

      // Try connecting to the server
      const net = require('node:net');
      const client = new net.Socket();

      await new Promise<void>((resolve, reject) => {
        client.connect(port, 'localhost', () => {
          resolve();
        });

        client.on('error', (err: Error) => {
          reject(err);
        });
      });

      client.destroy();
    });
    it('should emit connection event when client connects', async () => {
      await server.start(3102);

      const connectionPromise = new Promise<void>((resolve) => {
        server.on('connection', (_socket) => {
          resolve();
        });
      });

      //connect a client
      const net = require('node:net');
      const client = new net.Socket();
      client.connect(3102, 'localhost');

      await connectionPromise;

      client.destroy();
    });

    it('should track connected clients', async () => {
      await server.start(3103);

      //connect first client
      const client1 = new (require('node:net').Socket)();
      await new Promise<void>((resolve) => {
        client1.connect(3103, 'localhost', resolve);
      });
      expect(server.getClientCount()).toBe(1);

      //connect second client
      const client2 = new (require('node:net').Socket)();
      await new Promise<void>((resolve) => {
        client2.connect(3103, 'localhost', resolve);
      });
      expect(server.getClientCount()).toBe(2);

      //disconnect first client
      client1.destroy();
      await new Promise((resolve) => setTimeout(resolve, 50));

      expect(server.getClientCount()).toBe(1);

      //disconnect second client
      client2.destroy();
    });

    it('should assign unique session IDs to clients', async () => {
      await server.start(3104);

      const sessionIds: string[] = [];

      server.on('connection', (_socket, sessionId) => {
        console.log('Connection received, sessionId:', sessionId);
        sessionIds.push(sessionId);
      });

      // connect three clients
      const client1 = new (require('node:net').Socket)();
      const client2 = new (require('node:net').Socket)();
      const client3 = new (require('node:net').Socket)();

      await new Promise<void>((resolve) => {
        client1.connect(3104, 'localhost', resolve);
      });
      await new Promise<void>((resolve) => {
        client2.connect(3104, 'localhost', resolve);
      });
      await new Promise<void>((resolve) => {
        client3.connect(3104, 'localhost', resolve);
      });

      // Check if we have three unique session IDs
      expect(sessionIds).toHaveLength(3);

      // Check that all session IDs are defined
      sessionIds.forEach((id) => {
        expect(id).toBeDefined();
        expect(id).not.toBeNull();
        expect(typeof id).toBe('string');
        expect(id.length).toBeGreaterThan(0);
      });
      expect(new Set(sessionIds).size).toBe(3); // all session IDs should be unique

      // Clean up
      client1.end();
      client2.end();
      client3.end();

      // Wait a moment to ensure all events are processed
      await new Promise((resolve) => setTimeout(resolve, 100));
    }, 10000);
  });
});
