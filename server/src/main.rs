pub mod global_variables;
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
        async fn server_wrapper(config: &'static common::repositories_types::server::config::Config) {
            println!("trying to create postgres pool...");
            let pg_pool = sqlx::postgres::PgPoolOptions::new().connect(secrecy::ExposeSecret::expose_secret(app_state::GetDatabaseUrl::get_database_url(&config))).await.unwrap();//todo remove .unwrap()
            println!("create_table_if_not_exists...");
            common::repositories_types::server::routes::api::example::Example::create_table_if_not_exists(&pg_pool).await.unwrap();//todo remove .unwrap()
            // println!("trying to create redis session storage...");
            // let redis_session_storage = match {
            //     use common::common::config::try_get_redis_session_storage::TryGetRedisSessionStorage;
            //     config.try_get_redis_session_storage().await
            // } {
            //     Ok(redis_session_storage) => redis_session_storage,
            //     Err(error) => {
            //         return Err(Box::new(
            //             common::repositories_types::server::server_wrapper::ServerWrapperErrorNamed::TryGetRedisSessionStorage {
            //                 try_get_redis_session_storage: error,
            //                 code_occurence: error_occurence_lib::code_occurence!(),
            //             }
            //         ))
            //     },
            // };
            println!("trying to build server...");
            crate::try_build_server::try_build_server(
                pg_pool, // redis_session_storage,
                config,
            ).await.unwrap();
            println!("server running!");
        }
        runtime.block_on(server_wrapper(config));
    })
    .unwrap()
    .join()
    .unwrap();
}