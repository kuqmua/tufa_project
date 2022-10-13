use impl_display::ImplDisplay;
use tracing::dispatcher::SetGlobalDefaultError;
use tracing::log::SetLoggerError;

#[derive(thiserror::Error, Debug, ImplDisplay)]
pub enum InitSubcriberErrorEnum {
    SetLogger {
        #[from]
        source: SetLoggerError,
    },
    SetGlobalDefault {
        #[from]
        source: SetGlobalDefaultError,
    },
}
