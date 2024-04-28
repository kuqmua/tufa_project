pub fn entry(config: &'static common::repositories_types::server::config::Config) {
    match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
    {
        Err(error) => {
            panic!("tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build() failed, error: {error:#?}")
        }
        Ok(runtime) => {
            common::dev::dev();
            // runtime.block_on(crate::dev::dev());
            if let Err(error) =
                common::repositories_types::server::telemetry::init_subscriber::init_subscriber(
                    common::repositories_types::server::telemetry::get_subscriber::get_subscriber(
                        env!("CARGO_PKG_VERSION"),
                        config,
                        std::io::stdout,
                    ),
                )
            {
                panic!("common::repositories_types::server::telemetry::init_subscriber::init_subscriber failed, error: {error:#?}")
            } else {
                //preparation logic must be enabled by default. service must check on existing database tables.
                println!("checking net availability...");
                if let Err(error) = runtime.block_on(
                    common::server::net::net_check_availability::net_check_availability(config),
                ) {
                    error_occurence_lib::error_log::ErrorLog::error_log(&*error, &config);
                } else if let Err(error) =
                    runtime.block_on(crate::server_wrapper::server_wrapper(config))
                {
                    eprintln!("server stopped");
                    error_occurence_lib::error_log::ErrorLog::error_log(&*error, &config);
                }
                else {
                    //clippy lint forces to add empty else
                }
            }
        }
    }
}
