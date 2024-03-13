pub fn init_subscriber(
    subscriber: impl tracing::Subscriber + Send + Sync,
) -> Result<
    (),
    crate::repositories_types::server::telemetry::init_subcriber_error_enum::InitSubcriberErrorEnum,
> {
    if let Err(e) = tracing_log::LogTracer::init() {
        return Err(crate::repositories_types::server::telemetry::init_subcriber_error_enum::InitSubcriberErrorEnum::SetLogger {
            error: e,
            code_occurence: error_occurence_lib::code_occurence!(),
        });
    }
    if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
        return Err(crate::repositories_types::server::telemetry::init_subcriber_error_enum::InitSubcriberErrorEnum::SetGlobalDefault {
            error: e,
            code_occurence: error_occurence_lib::code_occurence!(),
        });
    }
    Ok(())
}
