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

pub fn validate_hmac(key: &HmacKey, data: &[u8], expected: &[u8]) -> bool {
    let mut mac =
        HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC can accept key of any size");
    mac.update(data);

    // Verify the HMAC
    mac.verify_slice(expected).is_ok()
}

pub fn extract_hmac_prefix(hmac: &[u8]) -> u16 {
    // Take first 2 types of HMAC and convert to u16
    if hmac.len() < 2 {
        return 0;
    }

    u16::from_be_bytes([hmac[0], hmac[1]])
}

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

    #[test]
    fn test_validate_hmac_success() {
        // Test validating a correct HMAC
        let key = HmacKey::from_bytes(b"Validation_Test_Key_32_Bytes!!!!");
        let message = b"Message to validate HMAC";
        let hmac = generate_hmac(&key, message);

        assert!(validate_hmac(&key, message, &hmac));
    }

    #[test]
    fn test_validate_hmac_failure() {
        // Test validating an incorrect HMAC
        let key = HmacKey::from_bytes(b"Validation_Test_Key_32_Bytes!!!!");
        let message = b"Message to validate HMAC";
        let wrong_hmac = vec![0u8; 32]; // Incorrect HMAC

        assert!(!validate_hmac(&key, message, &wrong_hmac));
    }

    #[test]
    fn test_extract_hmac_prefix() {
        // Test extracting 16-bit prefix from HMAC
        let key = HmacKey::from_bytes(b"Prefix_Extraction_Key_32_Bytes!!");
        let message = b"Message for HMAC prefix extraction";

        let full_hmac = generate_hmac(&key, message);
        let prefix = extract_hmac_prefix(&full_hmac);

        // Should dbe a u16 value
        assert!(prefix > 0);
        // Should match first 2 bytes of HMAC
        let expected_prefix = u16::from_be_bytes([full_hmac[0], full_hmac[1]]);
        assert_eq!(prefix, expected_prefix);
    }
}
