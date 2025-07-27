export interface AudioPacket {
  channelId: number;
  senderId: number;
  sequenceNumber: number;
  timestamp: number;
  signalStrength: number;
  frameDuration: number;
  audioDataLength: number;
  hmacPrefix: number;
  audioData: Buffer;
}

export function createAudioPacket(
  channelId: number,
  senderId: number,
  audioData: Buffer,
): AudioPacket {
  return {
    channelId,
    senderId,
    sequenceNumber: 0, // This should be set by the sender
    timestamp: Date.now(),
    signalStrength: 100, // Default signal strength
    frameDuration: 20, // Default frame duration in milliseconds
    audioDataLength: audioData.length,
    hmacPrefix: 0, // This should be set by the sender
    audioData,
  };
}
