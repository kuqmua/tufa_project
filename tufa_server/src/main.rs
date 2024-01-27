// #![deny(
//     clippy::indexing_slicing,
//     clippy::arithmetic_side_effects,
//     clippy::unwrap_used,
//     clippy::float_arithmetic
// )]
// #![allow(clippy::too_many_arguments)]

pub mod entry;
pub mod global_variables;
mod server_wrapper;
// #[cfg(test)]
// mod tests;

//query! containing mods
// pub mod authentication;
// pub mod idempotency;
// pub mod issue_delivery_worker;
pub mod routes;
pub mod try_build_server;

pub mod dev;

fn main() {
    println!(
        "project commit {}",
        tufa_common::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
            .project_commit
    );
    crate::entry::entry(
        crate::global_variables::runtime::config::CONFIG.get_or_init(|| {
            tufa_common::repositories_types::tufa_server::config::config_struct::Config::try_from(
        tufa_common::repositories_types::tufa_server::config::config_struct::ConfigUnchecked::new()
        .unwrap_or_else(|e| panic!("failed to ConfigUnchecked::new(), reason: {e:#?}"))
    ).unwrap_or_else(|e| panic!("failed to Config try_from ConfigUnchecked, reason: {e}"))
        }),
    );
}
