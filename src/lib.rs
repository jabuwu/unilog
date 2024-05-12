use tracing::Level;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn init(default_level: Level, default_filter: &str) {
    LogTracer::init().unwrap();
    let default_filter = format!("{},{}", default_level, default_filter);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(&default_filter))
        .unwrap();
    let subscriber = Registry::default().with(filter_layer);
    #[cfg(all(not(target_arch = "wasm32")))]
    {
        let fmt_layer = tracing_subscriber::fmt::Layer::default().with_writer(std::io::stderr);
        tracing::subscriber::set_global_default(subscriber.with(fmt_layer)).unwrap()
    }
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        tracing::subscriber::set_global_default(subscriber.with(tracing_wasm::WASMLayer::new(
            tracing_wasm::WASMLayerConfig::default(),
        )))
        .unwrap()
    }
}
