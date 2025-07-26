import {createAudioPacket, AudioPacket} from "./packet";

describe('AudioPacket', () => {
    describe('createAudioPacket()', () => {
        it('should create a valid audio packet with provided data', () => {
            // Arrange
            const channelId = 1;
            const senderId = 123;
            const audioData = Buffer.from([0x01, 0x02, 0x03, 0x04]);
            
            // Act
            const packet = createAudioPacket(channelId, senderId, audioData);
            packet.timestamp = Date.now(); // Set timestamp to current time
            
            // Assert
            expect(packet.channelId).toBe(channelId);
            expect(packet.senderId).toBe(senderId);
            expect(packet.audioData).toBe(audioData);
            expect(packet.audioData.length).toBe(4);
            expect(packet.signalStrength).toBe(100);
            expect(packet.frameDuration).toBe(20);
        });
        
        it('should set timestamp to current time', () => {
            // Arrange
            const beforeTime = Date.now();
            const audioData = Buffer.alloc(0);
            
            // Act
            const packet = createAudioPacket(1, 1, audioData);
            const afterTime = Date.now();
            
            // Assert
            expect(packet.timestamp).toBeGreaterThanOrEqual(beforeTime);
            expect(packet.timestamp).toBeLessThanOrEqual(afterTime);
        })
            
    })
})