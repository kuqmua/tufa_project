pub fn get_subscriber<Sink, Config: app_state::GetTracingLevel + std::marker::Send + std::marker::Sync>(name: &str, config: &Config, sink: Sink) -> impl tracing::Subscriber + Send + Sync
where
    // This "weird" syntax is a higher-ranked trait bound (HRTB)
    // It basically means that Sink implements the `MakeWriter`
    // trait for all choices of the lifetime parameter `'a`
    // Check out https://doc.rust-lang.org/nomicon/hrtb.html
    // for more details.
    Sink: for<'a> tracing_subscriber::fmt::MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(config.get_tracing_level().to_string()));
    let formatting_layer = tracing_bunyan_formatter::BunyanFormattingLayer::new(name.to_owned(), sink);
    {
        use tracing_subscriber::layer::SubscriberExt;
        tracing_subscriber::Registry::default().with(env_filter).with(tracing_bunyan_formatter::JsonStorageLayer).with(formatting_layer)
    }
}
