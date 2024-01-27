pub fn entry<'a>(
    config: &'static tufa_common::repositories_types::tufa_server::config::config_struct::Config,
) {
    match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
    {
        Err(e) => {
            panic!("tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build() failed, error: {e:#?}")
        }
        Ok(runtime) => {
            tufa_common::dev::dev();
            runtime.block_on(crate::dev::dev());
            if let Err(e) = tufa_common::repositories_types::tufa_server::telemetry::init_subscriber::init_subscriber(
                tufa_common::repositories_types::tufa_server::telemetry::get_subscriber::get_subscriber(
                env!("CARGO_PKG_VERSION"),
                config,
                std::io::stdout,
            ))
            {
                panic!("tufa_common::repositories_types::tufa_server::telemetry::init_subscriber::init_subscriber failed, error: {e:#?}")
            }
            else {
                //preparation logic must be enabled by default. service must check on existing database tables.
                println!("checking net availability...");
                if let Err(e) = runtime.block_on(tufa_common::server::net::net_check_availability::net_check_availability(config)) {
                    tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                        &*e,
                        &config
                    );
                }
                else {
                    if let Err(e) = runtime.block_on(crate::server_wrapper::server_wrapper(&config)) {
                        eprintln!("server stopped");
                        tufa_common::common::error_logs_logic::error_log::ErrorLog::error_log(
                            &*e,
                            &config
                        );
                    }
                }
            }
        }
    }
}
