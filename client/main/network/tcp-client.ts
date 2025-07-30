import { EventEmitter } from 'node:events';
import * as net from 'node:net';

export class TcpClient extends EventEmitter {
  private socket?: net.Socket;
  private connected: boolean = false;
  private sessionId?: string;
  private buffer: string = '';

  connect(host: string, port: number): void {
    this.socket = net.createConnection({ host, port });

    this.socket.on('connect', () => {
      this.connected = true;
      this.emit('connected');
      this.sendHandshake();
    });

    this.socket.on('error', (error) => {
      this.connected = false;
      this.emit('error', error);
    });

    this.socket.on('data', (data) => {
      this.handleData(data);
    });
  }

  private sendHandshake(): void {
    const handshakeMessage = {
      type: 'handshake',
      clientVersion: '1.0.0',
    };

    if (!this.socket) {
      throw new Error('Socket is not connected');
    }
    this.socket.write(`${JSON.stringify(handshakeMessage)} \n`);
  }

  private handleData(data: Buffer): void {
    this.buffer += data.toString();
    const lines = this.buffer.split('\n');
    this.buffer = lines.pop() || ''; // Keep the last incomplete line in buffer

    for (const line of lines) {
      if (line.trim()) {
        try {
          const message = JSON.parse(line);
          this.handleMessage(message);
        } catch (error) {
          this.emit('error', new Error(`Failed to parse message: ${error}`));
        }
      }
    }
  }

  private handleMessage(message: {
    sessionId: string;
    clientVersion: string;
    type: string;
  }): void {
    if (message.type === 'handshake_ack') {
      this.sessionId = message.sessionId;
      this.emit('handshake_complete', message);
    }
  }

  isConnected(): boolean {
    return this.connected;
  }

  getSessionId(): string | undefined {
    return this.sessionId;
  }

  disconnect(): void {
    if (this.socket) {
      this.connected = false;
      this.socket.end();
      this.socket = undefined;
      this.sessionId = '';
    }
  }
}
