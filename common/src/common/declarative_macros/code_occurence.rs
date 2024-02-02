#[macro_export]
macro_rules! code_occurence {
    ( $( $x:expr ),* ) => {{
        error_occurence_lib::code_occurence::CodeOccurence::new(
            $crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                .to_git_info_without_lifetime(),//todo remove it
            file!().to_string(),
            line!(),
            column!(),
        )
    }};
}