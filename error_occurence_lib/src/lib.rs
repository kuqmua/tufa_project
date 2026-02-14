pub mod code_occurence;

pub use ::to_err_string::ToStdStringString;
pub use error_occurence::ErrorOccurence;

#[macro_export]
macro_rules! code_occurence {
    ( $( $x:expr ),* ) => {{
        error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            None,
        )
    }};
}
