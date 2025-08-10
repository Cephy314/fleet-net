//! Test support utilities for Fleet Net
//!
//! This crate provides common test helpers and utilities that are shared across
//! the Fleet Net workspace for testing. It is not intended for production use.

pub mod crypto;
pub mod io;
pub mod net;
pub mod time;
pub mod tls;

// Re-export commonly used items at the crate root
pub use crypto::{generate_test_certs, init_crypto_once, TestCertBundle};
pub use net::{connected_tcp_pair, mock_connection_pair};
pub use time::{wait_until, with_timeout};
