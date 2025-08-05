//! Logging configuration for Fleet Net.
//!
//! This module provides centralized logging setup using the `tracing` ecosystem.
//! It configures structured logging with appropriate filtering for debugging
//! and production environments.

use tracing_subscriber;

/// Initializes the tracing/logging system for Fleet Net.
///
/// This function sets up the global tracing subscriber with:
/// - Formatted output to stdout
/// - Debug level logging for fleet_net modules
/// - Environment variable support for log filtering
///
/// # Environment Variables
///
/// The log level can be overridden using the `RUST_LOG` environment variable:
/// ```bash
/// RUST_LOG=trace cargo run  # More verbose
/// RUST_LOG=warn cargo run   # Less verbose
/// RUST_LOG=fleet_net=trace,tokio=warn cargo run  # Mixed levels
/// ```
///
/// # Panics
///
/// This function will panic if called more than once, as the global
/// tracing subscriber can only be set once per process.
///
/// # Examples
///
/// ```no_run
/// use fleet_net_common::logging::init_tracing;
///
/// // Initialize logging at program start
/// init_tracing();
///
/// tracing::info!("Fleet Net server starting...");
/// ```
pub fn init_tracing() {
    tracing_subscriber::fmt()
        // Set default filter to debug for fleet_net modules
        .with_env_filter("fleet_net=debug")
        .init();
}
