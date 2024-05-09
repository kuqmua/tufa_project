pub use error_occurence::ErrorOccurenceTest;
pub use ::to_std_string_string::ToStdStringString;
pub mod code_occurence;

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