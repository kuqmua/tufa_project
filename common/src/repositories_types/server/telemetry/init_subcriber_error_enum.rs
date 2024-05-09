#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
pub enum InitSubcriberErrorEnum {
    SetGlobalDefault {
        #[eo_to_std_string_string]
        error: tracing::dispatcher::SetGlobalDefaultError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SetLogger {
        #[eo_to_std_string_string]
        error: tracing::log::SetLoggerError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
