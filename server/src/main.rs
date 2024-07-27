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

fn main() {
    println!("commit {}", git_info::PROJECT_GIT_INFO.commit);
    let config = crate::global_variables::runtime::config::CONFIG.get_or_init(|| common::repositories_types::server::config::Config::try_from_env().unwrap());
    crate::entry::entry(config);
}
