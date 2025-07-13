pub mod global_variables;
pub mod routes;
// #[cfg(test)]
// mod tests;
//query! containing mods
// pub mod authentication;
// pub mod idempotency;
// pub mod issue_delivery_worker;

fn main() {
    std::thread::Builder::new().stack_size(16 * 1024 * 1024) // 16 MB
    .spawn(|| {
        tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            println!("commit {}", git_info::PROJECT_GIT_INFO.commit);
            let config = crate::global_variables::runtime::config::CONFIG.get_or_init(|| common::repositories_types::server::config::Config::try_from_env().unwrap());
            // if let Err(error) = common::repositories_types::server::telemetry::init_subscriber::init_subscriber(common::repositories_types::server::telemetry::get_subscriber::get_subscriber(env!("CARGO_PKG_VERSION"), config, std::io::stdout)) {
            //     panic!("common::repositories_types::server::telemetry::init_subscriber::init_subscriber failed, error: {error:#?}")
            // }
            println!("trying to create postgres pool...");
            let postgres_pool = sqlx::postgres::PgPoolOptions::new().connect(secrecy::ExposeSecret::expose_secret(app_state::GetDatabaseUrl::get_database_url(&config))).await.unwrap();
            common::repositories_types::server::routes::api::example::Example::prepare_postgresql(&postgres_pool).await.unwrap();
            // todo preparation logic must be enabled by default. service must check on existing database tables.
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
            let service_socket_address = app_state::GetServiceSocketAddress::get_service_socket_address(config);
            println!("trying to up server on {service_socket_address}");
            let app_state = std::sync::Arc::new(common::repositories_types::server::routes::app_state::AppState {
                postgres_pool,
                config,
                project_git_info: &git_info::PROJECT_GIT_INFO,
            });
            axum::serve(
                tokio::net::TcpListener::bind(service_socket_address).await.unwrap(),
                axum::Router::new()
                    .merge(common::server::routes::routes(std::sync::Arc::<common::repositories_types::server::routes::app_state::AppState<'_>>::clone(&app_state)))
                    .merge(common::repositories_types::server::routes::api::example::Example::routes(std::sync::Arc::<common::repositories_types::server::routes::app_state::AppState<'_>>::clone(&app_state)))
                    .merge(common::server::routes::not_found::not_found_route(std::sync::Arc::<common::repositories_types::server::routes::app_state::AppState<'_>>::clone(&app_state)))
                    // .fallback_service(routes_static())
                    .layer(
                        tower_http::cors::CorsLayer::new()
                        // .allow_methods([
                        //     http::Method::GET,
                        //     http::Method::POST,
                        //     http::Method::PATCH,
                        //     http::Method::DELETE,
                        // ])
                        .allow_origin(["http://127.0.0.1".parse().unwrap()]),
                    )
                    .merge(utoipa_swagger_ui::SwaggerUi::new(constants::SLASH_SWAGGER_UI).url("/api-docs/openapi.json", {
                        #[derive(utoipa::OpenApi)]
                        #[openapi(
                            paths(
                                common::server::routes::git_info::git_info,
                                // common::repositories_types::server::routes::api::cats::create_many,
                                // common::repositories_types::server::routes::api::cats::create_one,
                                // common::repositories_types::server::routes::api::cats::read_many,
                                // common::repositories_types::server::routes::api::cats::read_one,
                                // common::repositories_types::server::routes::api::cats::update_many,
                                // common::repositories_types::server::routes::api::cats::update_one,
                                // common::repositories_types::server::routes::api::cats::delete_many,
                                // common::repositories_types::server::routes::api::cats::delete_one
                            ),
                            components(
                                    schemas(
                                        common::server::routes::git_info::GitInfo,                          

                                        common::common::utoipa::std::time::StdTimeDuration,
                                        error_occurence_lib::code_occurence::CodeOccurence,
                                        //
                                        // postgresql_crud::TimeMonth,
                                        // postgresql_crud::SqlxTypesTimeUtcOffset,
                                        // postgresql_crud::NumBigintSign,
                                        // postgresql_crud::NumBigintBigInt,
                                        // postgresql_crud::StdPrimitiveBool,
                                        // postgresql_crud::StdPrimitiveI16,
                                        // postgresql_crud::StdPrimitiveI32,
                                        // postgresql_crud::StdPrimitiveI64,
                                        // postgresql_crud::StdPrimitiveF32,
                                        // postgresql_crud::StdPrimitiveF64,
                                        // postgresql_crud::StdStringString,
                                        // postgresql_crud::StdVecVecStdPrimitiveU8,
                                        // postgresql_crud::SqlxPostgresTypesPgInterval,
                                        // postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI64,
                                        // postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI32,
                                        // postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
                                        // postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
                                        // //todo check all types and type decl order
                                        // postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
                                        // postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
                                        // postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
                                        // postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
                                        // postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
                                        // postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesDecimal,
                                        // postgresql_crud::SqlxPostgresTypesPgMoney,
                                        // postgresql_crud::SqlxPostgresTypesPgCiText,
                                        // postgresql_crud::SqlxTypesBigDecimal,
                                        // postgresql_crud::SqlxTypesDecimal,
                                        // //todo
                                        // postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
                                        // postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
                                        // postgresql_crud::SqlxTypesChronoNaiveDateTime,
                                        // postgresql_crud::SqlxTypesChronoNaiveDate,
                                        // postgresql_crud::SqlxTypesChronoNaiveTime,
                                        // postgresql_crud::SqlxPostgresTypesPgTimeTz,
                                        // postgresql_crud::SqlxTypesTimePrimitiveDateTime,
                                        // postgresql_crud::SqlxTypesTimeOffsetDateTime,
                                        // postgresql_crud::SqlxTypesTimeDate,
                                        // postgresql_crud::SqlxTypesTimeTime,
                                        // postgresql_crud::SqlxTypesUuidUuid,
                                        // postgresql_crud::SqlxTypesIpnetworkIpNetwork,
                                        // postgresql_crud::StdNetIpAddr,
                                        // postgresql_crud::SqlxTypesMacAddressMacAddress,
                                        // postgresql_crud::SqlxTypesBitVec,
                                        // // postgresql_crud::SqlxTypesJson,//todo what to do with generics?
                                        // postgresql_crud::SerdeJsonValue,
                                    )
                            ),
                            modifiers(&SecurityAddon),
                            tags((name = "server", description = "server api"))
                        )] //todo - this thing actually using builder pattern. maybe generate builder in GeneratePostgresqlCrud then merge it together?
                        struct ApiDoc;
                        struct SecurityAddon;
                        impl utoipa::Modify for SecurityAddon {
                            fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
                                if let Some(components) = openapi.components.as_mut() {
                                    components.add_security_scheme("api_key", utoipa::openapi::security::SecurityScheme::ApiKey(utoipa::openapi::security::ApiKey::Header(utoipa::openapi::security::ApiKeyValue::new("todo_apikey"))));
                                }
                            }
                        }
                        <ApiDoc as utoipa::OpenApi>::openapi()
                    }))
                    .into_make_service(),
            )
            .await
            .unwrap_or_else(|error| panic!("axum builder serve await failed {error:#?}"));
        });
    })
    .unwrap()
    .join()
    .unwrap();
}