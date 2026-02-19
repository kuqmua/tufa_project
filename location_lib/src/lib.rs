pub mod code_occurence;
pub use ::to_err_string::ToErrString;
pub use location::Location;
#[macro_export]
macro_rules! code_occurence {
    ( $( $x:expr ),* ) => {{
        location_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            None,
        )
    }};
}
