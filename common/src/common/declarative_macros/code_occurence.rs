#[macro_export]
macro_rules! code_occurence {
    ( $( $x:expr ),* ) => {{
        error_occurence_lib::code_occurence::CodeOccurence::new(
            git_info::PROJECT_GIT_INFO
                .commit
                .to_string(), //todo maybe put struct, but dont want to deal with lifetimes
            file!().to_string(),
            line!(),
            column!(),
            None,
        )
    }};
}
//crate::global_variables::compile_time::project_
