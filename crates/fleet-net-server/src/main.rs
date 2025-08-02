use tracing::info;

fn main() {
    // Initialize tracing for logging
    fleet_net_common::logging::init_tracing();

    info!("Starting Fleet Net Server...");
}
