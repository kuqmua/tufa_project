pub fn init_subscriber<T: tracing::Subscriber + Send + Sync>(subscriber: T) -> Result<(), crate::repositories_types::server::telemetry::init_subcriber_error_enum::InitSubcriberErrorEnum> {
    if let Err(error) = tracing_log::LogTracer::init() {
        return Err(crate::repositories_types::server::telemetry::init_subcriber_error_enum::InitSubcriberErrorEnum::SetLogger { error, code_occurence: error_occurence_lib::code_occurence!() });
    }
    if let Err(error) = tracing::subscriber::set_global_default(subscriber) {
        return Err(crate::repositories_types::server::telemetry::init_subcriber_error_enum::InitSubcriberErrorEnum::SetGlobalDefault { error, code_occurence: error_occurence_lib::code_occurence!() });
    }
    Ok(())
}
