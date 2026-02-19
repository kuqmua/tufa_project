pub mod code_occurence;
pub use ::to_err_string::ToErrString;
pub use er_occurence::ErOccurence;
#[macro_export]
macro_rules! code_occurence {
    ( $( $x:expr ),* ) => {{
        er_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            None,
        )
    }};
}
