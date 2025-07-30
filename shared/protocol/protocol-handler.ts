import { Packr, Unpackr } from 'msgpackr';
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
    return this.packer.encode(message);
  }

  decode(data: Buffer): Message {
    return this.unpacker.decode(data);
  }

  addData(data: Buffer): Message[] {
    this.buffer = Buffer.concat([this.buffer, data]);
    const messages: Message[] = [];

    while (this.buffer.length > 0) {
      try {
        // Try to decode a message.
        const message = this.unpacker.decode(this.buffer);
        messages.push(message);

        // Move past the decoded message.
        const bytesConsumed = this.unpacker.position || 0;
        this.buffer = this.buffer.slice(bytesConsumed);
      } catch (error) {
        // Not enough data for a complete message!
        break;
      }
    }

    return messages;
  }

  // Reset the buffer (useful for connection resets or when starting a new session)
  reset(): void {
    this.buffer = Buffer.alloc(0);
  }
}
