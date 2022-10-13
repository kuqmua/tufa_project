use crate::telemetry::init_subcriber_error_enum::InitSubcriberErrorEnum;
use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_log::LogTracer;

pub fn init_subscriber(
    subscriber: impl Subscriber + Send + Sync,
) -> Result<(), InitSubcriberErrorEnum> {
    LogTracer::init()?;
    set_global_default(subscriber)?;
    Ok(())
}
