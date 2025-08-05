use bytes::{Buf, BufMut, BytesMut};
use fleet_net_common::types::{ChannelId, UserId};
use std::borrow::Cow;
use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketError {
    #[error("Packet too short, expected at least 16 bytes")]
    TooShort,
    #[error("Invalid packet length, expected {expected} bytes but got {actual}")]
    InvalidLength { expected: usize, actual: usize },
    #[error("Invalid packet header")]
    InvalidFormat,
}

impl From<PacketError> for fleet_net_common::error::FleetNetError {
    fn from(err: PacketError) -> Self {
        fleet_net_common::error::FleetNetError::PacketError(Cow::Owned(err.to_string()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PacketHeader {
    /// Channel ID where audio is being sent.
    pub channel_id: ChannelId,

    /// User ID of the sender.
    pub user_id: UserId,

    /// Sequence number for the packet ordering (byte 4-5).
    pub sequence: u16,

    /// Relative timestamp in milliseconds (bytes 6-9).
    pub timestamp: u32,

    /// Signal strength of the sender 0 - 255 (byte 10).
    pub signal_strength: u8,

    /// Frame duration in ms (byte 11).
    pub frame_duration: u8,

    /// Audio data length in bytes (bytes 12-13).
    pub audio_length: u16,

    /// HMAC prefix - first 16 bits of HMAC-SHA256 (bytes 14-15).
    pub hmac_prefix: u16,
}

impl PacketHeader {
    pub const SIZE: usize = 16; // Total size of the header in bytes

    pub fn write_to<B: BufMut>(&self, buf: &mut B) {
        buf.put_u16(self.channel_id);
        buf.put_u16(self.user_id);
        buf.put_u16(self.sequence);
        buf.put_u32(self.timestamp);
        buf.put_u8(self.signal_strength);
        buf.put_u8(self.frame_duration);
        buf.put_u16(self.audio_length);
        buf.put_u16(self.hmac_prefix);
    }

    pub fn read_from<B: Buf>(buf: &mut B) -> Result<Self, PacketError> {
        if buf.remaining() < Self::SIZE {
            return Err(PacketError::TooShort);
        }

        Ok(PacketHeader {
            channel_id: buf.get_u16(),
            user_id: buf.get_u16(),
            sequence: buf.get_u16(),
            timestamp: buf.get_u32(),
            signal_strength: buf.get_u8(),
            frame_duration: buf.get_u8(),
            audio_length: buf.get_u16(),
            hmac_prefix: buf.get_u16(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AudioPacket {
    pub header: PacketHeader,
    pub opus_payload: Vec<u8>,
}

impl AudioPacket {
    /// Serialize back to bytes for the network transmission.
    pub fn to_bytes(&self) -> BytesMut {
        // create a buffer with enough space for the header and payload
        let mut buf = BytesMut::with_capacity(PacketHeader::SIZE + self.opus_payload.len());

        // Write the header first
        self.header.write_to(&mut buf);

        // Then write the opus payload
        buf.put_slice(&self.opus_payload);

        // return the buffer
        buf
    }

    /// Parse packet from network bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, PacketError> {
        let mut buf = bytes::Bytes::copy_from_slice(data);

        // Parse the header
        let header = PacketHeader::read_from(&mut buf)?;

        // Verify payload length
        if buf.remaining() != header.audio_length as usize {
            return Err(PacketError::InvalidLength {
                expected: header.audio_length as usize,
                actual: buf.remaining(),
            });
        }

        // Extract the opus payload
        let opus_payload = buf.to_vec();

        // Return the constructed AudioPacket
        Ok(AudioPacket {
            header,
            opus_payload,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_round_trip() {
        let header = PacketHeader {
            channel_id: 0x1234,
            user_id: 0x5678,
            sequence: 0x9ABC,
            timestamp: 0xDEADBEEF,
            signal_strength: 200,
            frame_duration: 20,
            audio_length: 10,
            hmac_prefix: 0xCAFE,
        };

        let payload = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let packet = AudioPacket {
            header,
            opus_payload: payload,
        };

        // Serialize to bytes
        let bytes = packet.to_bytes();

        // Deserialize back to AudioPacket
        let parsed_packet = AudioPacket::from_bytes(&bytes).unwrap();

        // Verify
        assert_eq!(parsed_packet.header.channel_id, header.channel_id);
        assert_eq!(parsed_packet.header.user_id, header.user_id);
        assert_eq!(parsed_packet.opus_payload, packet.opus_payload);
    }
}
