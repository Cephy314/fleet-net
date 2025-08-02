pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter("fleet_net=debug")
        .init();
}