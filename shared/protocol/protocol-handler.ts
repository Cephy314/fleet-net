import { Packr, Unpackr, unpackMultiple } from 'msgpackr';
import type { Message } from './messages';

export class ProtocolHandler {
  private packer: Packr;
  private unpacker: Unpackr;
  private buffer: Buffer = Buffer.alloc(0); // Initialize an empty buffer to store incoming data

  constructor() {
    const options = {
      structures: [], // Will be populated with recurring structures
      maxSharedStructures: 100, // Allow up to 100 shared structures
      useRecords: true, // Enable record extension for better performance
      moreTypes: true, // Support Sets, Maps, Errors etc.
    };

    this.packer = new Packr(options);
    this.unpacker = new Unpackr(options);
  }

  encode(message: Message): Buffer {
    const msgBuffer = Buffer.from(this.packer.encode(message));
    const lengthBuffer = Buffer.allocUnsafe(4);
    lengthBuffer.writeUInt32BE(msgBuffer.length, 0);
    return Buffer.concat([lengthBuffer, msgBuffer]);
  }

  decode(data: Buffer): Message {
    return this.unpacker.decode(data);
  }

  addData(data: Buffer): Message[] {
    this.buffer = Buffer.concat([this.buffer, data]);
    const messages: Message[] = [];

    while (this.buffer.length >= 4) {
      // Read the message length
      const messageLength = this.buffer.readUint32BE(0);

      // Check if we have the complete message
      if (this.buffer.length < 4 + messageLength) {
        break; // wait for more data
      }

      const messageBuffer = this.buffer.subarray(4, 4 + messageLength);
      try {
        const message = this.unpacker.decode(messageBuffer);
        messages.push(message);
      } catch (error) {
        console.error('Failed to decode message:', error);
        // skip this message and continue
      }

      // Remove the processed message from the buffer
      this.buffer = this.buffer.subarray(4 + messageLength);
    }
    return messages;
  }

  // Reset the buffer (useful for connection resets or when starting a new session)
  reset(): void {
    this.buffer = Buffer.alloc(0);
  }
}
