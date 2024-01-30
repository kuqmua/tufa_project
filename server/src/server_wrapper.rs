pub async fn server_wrapper<'a>(
    config: &'static common::repositories_types::server::config::config_struct::Config,
) -> Result<(), Box<common::repositories_types::server::server_wrapper::ServerWrapperErrorNamed>> {
    let postgres_pool = match common::common::config::try_get_postgres_pool::TryGetPostgresPool::try_get_postgres_pool(config).await {
        Ok(postgres_pool) => postgres_pool,
        Err(e) => {
            return Err(Box::new(
                common::repositories_types::server::server_wrapper::ServerWrapperErrorNamed::TryGetPostgresPool {
                    try_get_postgres_pool: e,
                    code_occurence: common::code_occurence!(),
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
    //     Err(e) => {
    //         return Err(Box::new(
    //             common::repositories_types::server::server_wrapper::ServerWrapperErrorNamed::TryGetRedisSessionStorage {
    //                 try_get_redis_session_storage: e,
    //                 code_occurence: common::code_occurence!(),
    //             }
    //         ))
    //     },
    // };
    println!("trying to build server...");
    if let Err(e) = crate::try_build_server::try_build_server(
        postgres_pool,
        // redis_session_storage,
        config,
    )
    .await
    {
        return Err(Box::new(common::repositories_types::server::server_wrapper::ServerWrapperErrorNamed::BuildServer {
            build_server: *e,
            code_occurence: common::code_occurence!(),
        }));
    }
    println!("server running!");
    Ok(())
}
