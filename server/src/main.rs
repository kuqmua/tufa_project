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
    std::thread::Builder::new().stack_size(16 * 1024 * 1024) // 16 MB
    .spawn(|| {
        println!("commit {}", git_info::PROJECT_GIT_INFO.commit);
        let config = crate::global_variables::runtime::config::CONFIG.get_or_init(|| common::repositories_types::server::config::Config::try_from_env().unwrap());
        let runtime = match tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build() {
            Ok(value) => value,
            Err(error) => panic!("tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build() failed, error: {error:#?}"),
        };
        if let Err(error) = common::repositories_types::server::telemetry::init_subscriber::init_subscriber(common::repositories_types::server::telemetry::get_subscriber::get_subscriber(env!("CARGO_PKG_VERSION"), config, std::io::stdout)) {
            panic!("common::repositories_types::server::telemetry::init_subscriber::init_subscriber failed, error: {error:#?}")
        }
        //preparation logic must be enabled by default. service must check on existing database tables.
        // println!("checking net availability...");
        // if let Err(error) = runtime.block_on(common::server::net::net_check_availability::net_check_availability(config)) {
        //     eprintln!("{error}");
        // }
        if let Err(error) = runtime.block_on(crate::server_wrapper::server_wrapper(config)) {
            eprintln!("server stopped");
            eprintln!("{error}");
        }
    })
    .unwrap()
    .join()
    .unwrap();
}