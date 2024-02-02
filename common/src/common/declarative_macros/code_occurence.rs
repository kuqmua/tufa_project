#[macro_export]
macro_rules! code_occurence {
    ( $( $x:expr ),* ) => {{
        error_occurence_lib::code_occurence::CodeOccurence::new(
            $crate::global_variables::compile_time::git_info::GIT_INFO
                .to_git_info_without_lifetime(),
            file!().to_string(),
            line!(),
            column!(),
        )
    }};
}