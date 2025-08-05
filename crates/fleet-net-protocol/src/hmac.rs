use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub struct HmacKey {
    key: [u8; 32], // HMAC key must be 32 bytes for SHA-256
}

impl HmacKey {
    fn new(key: &[u8; 32]) -> Self {
        HmacKey {
            key: *key, // Dereference the slice to get the array
        }
    }

    pub fn from_bytes(bytes: &[u8; 32]) -> HmacKey {
        HmacKey::new(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.key
    }
}

pub fn generate_hmac(key: &HmacKey, data: &[u8]) -> Vec<u8> {
    // Create HMAC instance from key bytes.
    let mut mac =
        HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC can accept key of any size");

    // Process the data
    mac.update(data);

    // Get the result and conver to Vec<u8>
    let result = mac.finalize();
    result.into_bytes().to_vec()
}

pub fn validate_hmac() {}

pub fn extract_hmac_prefix() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmac_key_creation() {
        let key_bytes = b"test_session_key_32_bytes_long!!";
        let hmac_key = HmacKey::from_bytes(key_bytes);

        // Should be able to get the key back
        assert_eq!(hmac_key.as_bytes(), key_bytes);
    }

    #[test]
    fn test_generate_hmac() {
        let key = HmacKey::from_bytes(b"test_session_key_32_bytes_long!!");
        let message = b"{'type': 'join_channel', 'channel_id': 12345}";

        let hmac = generate_hmac(&key, message);

        // HMAC-SHA256 should produce 32 bytes.
        assert_eq!(hmac.len(), 32);
    }
}
