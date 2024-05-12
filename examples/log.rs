use tracing::{info, Level};

fn main() {
    unilog::init(Level::INFO, "");
    info!("hello");
}
