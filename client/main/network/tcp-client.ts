import { EventEmitter } from 'node:events';
import * as net from 'node:net';
import {
  type HandshakeAckMessage,
  type Message,
  MessageType,
} from '../../../shared/protocol/messages';
import { ProtocolHandler } from '../../../shared/protocol/protocol-handler';

export class TcpClient extends EventEmitter {
  private socket?: net.Socket;
  private connected: boolean = false;
  private sessionId?: string;
  private protocolHandler: ProtocolHandler;

  constructor() {
    super();
    this.protocolHandler = new ProtocolHandler();
  }

  /**
   * Connects to a TCP server at the specified host and port.
   * @param host - The hostname or IP address of the server.
   * @param port - The port number on which the server is listening.
   */
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

  /**
   * Sends a handshake message to the server.
   */
  private sendHandshake(): void {
    const handshakeMessage = {
      type: MessageType.HANDSHAKE,
      clientVersion: '1.0.0',
    };

    if (!this.socket) {
      throw new Error('Socket is not connected');
    }
    const encoded = this.protocolHandler.encode(handshakeMessage);
    this.socket.write(encoded);
  }

  /**
   * Handles incoming data from the server.
   * @param data - The data received from the server.
   */
  private handleData(data: Buffer): void {
    const messages = this.protocolHandler.addData(data);

    for (const message of messages) {
      this.handleMessage(message);
    }
  }

  /**
   * Processes a single message received from the server.
   * @param message - The message to process.
   */
  private handleMessage(message: Message): void {
    if (message.type === MessageType.HANDSHAKE_ACK) {
      const ackMessage = message as HandshakeAckMessage;
      this.sessionId = ackMessage.sessionId;
      this.emit('handshake_complete', ackMessage);
    }
  }

  /**
   * Checks if the client is currently connected to the server.
   * @returns {boolean} - True if connected, false otherwise.
   */
  isConnected(): boolean {
    return this.connected;
  }

  /**
   * Retrieves the session ID assigned by the server.
   * @returns {string | undefined} - The session ID, or undefined if not set.
   */
  getSessionId(): string | undefined {
    return this.sessionId;
  }

  /**
   * Disconnects from the server and cleans up resources.
   */
  disconnect(): void {
    if (this.socket) {
      this.connected = false;
      this.socket.end();
      this.socket = undefined;
      this.sessionId = '';
      this.protocolHandler.reset(); // Reset protocol handler state
    }
  }
}
