import { randomBytes } from 'node:crypto';
import { EventEmitter } from 'node:events';
import * as net from 'node:net';

export class TcpServer extends EventEmitter {
  private server?: net.Server;
  private port: number = -1;
  private clients: Set<net.Socket> = new Set();

  async start(port: number): Promise<void> {
    return new Promise((resolve, reject) => {
      this.server = net.createServer();

      // Handle incoming connections
      this.server.on('connection', (socket) => {
        this.clients.add(socket);

        const sessionId = randomBytes(16).toString('hex');

        // Handle client disconnection
        socket.on('close', () => {
          this.clients.delete(socket);
        });

        // Handle errors on the socket
        socket.on('error', (err) => {
          console.error('Socket error:', err);
          this.clients.delete(socket);
        });

        // Emit the connection event
        this.emit('connection', socket, sessionId);
      });

      this.server.listen(port, () => {
        this.port = port;
        resolve();
      });

      this.server.on('error', reject);
    });
  }

  async stop(): Promise<void> {
    return new Promise((resolve, _reject) => {
      if (this.server) {
        this.server.close(() => {
          this.server = undefined;
          this.port = -1;
          resolve();
        });
      } else {
        resolve();
      }
    });
  }

  isListening(): boolean {
    if (this.server) {
      return this.server.listening;
    }
    return false;
  }

  getPort(): number {
    return this.port;
  }

  getClientCount(): number {
    return this.clients.size;
  }
}
