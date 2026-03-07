pub mod loc;
pub use ::to_err_string::ToErrString;
pub use location::Location;
pub use optml::Optml;
#[macro_export]
macro_rules! loc {
    ( $( $x:expr ),* ) => {{ location_lib::loc::Loc::new(file!().to_owned(), line!(), column!(), None) }};
}
