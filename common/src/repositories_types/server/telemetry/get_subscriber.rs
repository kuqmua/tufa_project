pub fn get_subscriber<Sink, Config: app_state::GetTracingLevel + Send + Sync>(name: &str, config: &Config, sink: Sink) -> impl tracing::Subscriber + Send + Sync
where
    // This "weird" syntax is a higher-ranked trait bound (HRTB)
    // It basically means that Sink implements the `MakeWriter`
    // trait for all choices of the lifetime parameter `'lifetime`
    // Check out https://doc.rust-lang.org/nomicon/hrtb.html
    // for more details.
    Sink: for<'lifetime> tracing_subscriber::fmt::MakeWriter<'lifetime> + Send + Sync + 'static,
{
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(config.get_tracing_level().to_string()));
    let formatting_layer = tracing_bunyan_formatter::BunyanFormattingLayer::new(name.to_owned(), sink);
    { tracing_subscriber::layer::SubscriberExt::with(tracing_subscriber::layer::SubscriberExt::with(tracing_subscriber::layer::SubscriberExt::with(tracing_subscriber::Registry::default(), env_filter), tracing_bunyan_formatter::JsonStorageLayer), formatting_layer) }
}
