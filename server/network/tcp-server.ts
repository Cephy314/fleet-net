import { randomBytes } from 'node:crypto';
import { EventEmitter } from 'node:events';
import * as net from 'node:net';
import type { HandshakeMessage, Message } from '../../shared/protocol/messages';
import { MessageType } from '../../shared/protocol/messages';
import { ProtocolHandler } from '../../shared/protocol/protocol-handler';

interface ClientConnection {
  socket: net.Socket;
  sessionId: string;
  protocolHandler: ProtocolHandler;
}

export class TcpServer extends EventEmitter {
  private server?: net.Server;
  private port: number = -1;
  private clients: Map<net.Socket, ClientConnection> = new Map();

  async start(port: number): Promise<void> {
    return new Promise((resolve, reject) => {
      this.server = net.createServer();

      // Handle incoming connections
      this.server.on('connection', (socket) => {
        const sessionId = randomBytes(16).toString('hex');
        const protocolHandler = new ProtocolHandler();

        const client: ClientConnection = {
          socket,
          sessionId,
          protocolHandler,
        };

        this.clients.set(socket, client);

        // Handle incoming data from the client
        socket.on('data', (data) => {
          this.handleClientData(client, data);
        });

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

  private handleClientData(client: ClientConnection, data: Buffer) {
    const messages = client.protocolHandler.addData(data);

    for (const message of messages) {
      this.handleMessage(client, message);
    }
  }

  private handleMessage(client: ClientConnection, message: Message): void {
    if (message.type === MessageType.HANDSHAKE) {
      // Send handshake acknowledgement
      const response = {
        type: MessageType.HANDSHAKE_ACK,
        sessionId: client.sessionId,
        serverVersion: '1.0.0', // Example version, replace with actual server
      };

      const encoded = client.protocolHandler.encode(response);
      client.socket.write(encoded);
    }
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
