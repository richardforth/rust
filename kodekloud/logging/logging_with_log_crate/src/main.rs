// log crate
use log::{debug, error, info, warn};

fn main() {
    // RUST_LOG=debug cargo run to see more than just error!() logs to the console
    env_logger::init();

    info!("This is an info message");
    warn!("This is a warning");
    error!("This is an error");
    debug!("This is a debug message");
}

