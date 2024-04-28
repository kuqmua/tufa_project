pub(crate) async fn server_wrapper(
    config: &'static common::repositories_types::server::config::Config,
) -> Result<(), Box<common::repositories_types::server::server_wrapper::ServerWrapperErrorNamed>> {
    let postgres_pool = match common::common::config::try_get_postgres_pool::TryGetPostgresPool::try_get_postgres_pool(config).await {
        Ok(postgres_pool) => postgres_pool,
        Err(error) => {
            return Err(Box::new(
                common::repositories_types::server::server_wrapper::ServerWrapperErrorNamed::TryGetPostgresPool {
                    try_get_postgres_pool: error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                }
            ))
        },
    };
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
    if let Err(error) = crate::try_build_server::try_build_server(
        postgres_pool,
        // redis_session_storage,
        config,
    )
    .await
    {
        return Err(Box::new(common::repositories_types::server::server_wrapper::ServerWrapperErrorNamed::BuildServer {
            build_server: *error,
            code_occurence: error_occurence_lib::code_occurence!(),
        }));
    }
    println!("server running!");
    Ok(())
}
