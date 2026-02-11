use app_state::{GetDatabaseUrl, GetServiceSocketAddress};
use common_routes::common_routes;
use git_info::PROJECT_GIT_INFO;
use secrecy::ExposeSecret;
use server_app_state::ServerAppState;
use server_config::Config;
use server_table_example::TableExample;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::{net::TcpListener, runtime::Builder};
use tower_http::cors::CorsLayer;
use tracing_subscriber::fmt::init;
fn main() {
    init();
    Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
        .expect("5995c954-bb76-4620-b819-2b26f4b8f728")
        .block_on(async {
            let config = Config::try_from_env().expect("d74a6e5f-069a-49ea-9bac-19512e7b2bc5");
            let postgres_pool = PgPoolOptions::new()
                .max_connections(50)
                .connect(ExposeSecret::expose_secret(
                    GetDatabaseUrl::get_database_url(&config),
                ))
                .await
                .expect("8b72f688-be7d-4f5c-9185-44a27290a9d0");
            TableExample::prepare_postgresql(&postgres_pool)
                .await
                .expect("647fa499-c465-432d-ba4a-498f3e943ada");
            let tcp_listener =
                TcpListener::bind(GetServiceSocketAddress::get_service_socket_address(&config))
                    .await
                    .expect("3f294e7c-3386-497f-b76c-c0364d59a60d");
            let app_state = Arc::new(ServerAppState {
                postgres_pool,
                config,
                project_git_info: &PROJECT_GIT_INFO,
            });
            axum::serve(
                tcp_listener,
                axum::Router::new()
                    .merge(common_routes(Arc::<ServerAppState<'_>>::clone(&app_state)))
                    .merge(TableExample::routes(Arc::<ServerAppState<'_>>::clone(
                        &app_state,
                    )))
                    .layer(
                        CorsLayer::new()
                            // .allow_methods([
                            //     http::Method::GET,
                            //     http::Method::POST,
                            //     http::Method::PATCH,
                            //     http::Method::DELETE,
                            // ])
                            .allow_origin(["http://127.0.0.1"
                                .parse()
                                .expect("2a0b7c30-d4ba-4ce9-9fa9-98e981782191")]),
                    )
                    //todo partialy move to generate postresql crud implementation (except git_info route)
                    // .merge(utoipa_swagger_ui::SwaggerUi::new(constants::SLASH_SWAGGER_UI).url("/api-docs/openapi.json", {
                    //         // error: needless use of `for_each`
                    //         // #[derive(utoipa::OpenApi)]
                    //     //     #[openapi(
                    //     //     paths(
                    //     //         server_table_example::server::routes::git_info::git_info,
                    //     //         // server_table_example::repositories_types::server::routes::api::cats::create_many,
                    //     //         // server_table_example::repositories_types::server::routes::api::cats::create_one,
                    //     //         // server_table_example::repositories_types::server::routes::api::cats::read_many,
                    //     //         // server_table_example::repositories_types::server::routes::api::cats::read_one,
                    //     //         // server_table_example::repositories_types::server::routes::api::cats::update_many,
                    //     //         // server_table_example::repositories_types::server::routes::api::cats::update_one,
                    //     //         // server_table_example::repositories_types::server::routes::api::cats::delete_many,
                    //     //         // server_table_example::repositories_types::server::routes::api::cats::delete_one
                    //     //     ),
                    //     //     components(
                    //     //             schemas(
                    //     //                 server_table_example::server::routes::git_info::GitInfo,
                    //     //                 error_occurence_lib::code_occurence::StdTimeDuration,
                    //     //                 error_occurence_lib::code_occurence::CodeOccurence,
                    //     //                 //
                    //     //                 // postgresql_crud::TimeMonth,
                    //     //                 // postgresql_crud::SqlxTypesTimeUtcOffset,
                    //     //                 // postgresql_crud::NumBigintSign,
                    //     //                 // postgresql_crud::NumBigintBigInt,
                    //     //                 // postgresql_crud::StdPrimitiveBool,
                    //     //                 // postgresql_crud::StdPrimitiveI16,
                    //     //                 // postgresql_crud::StdPrimitiveI32,
                    //     //                 // postgresql_crud::StdPrimitiveI64,
                    //     //                 // postgresql_crud::StdPrimitiveF32,
                    //     //                 // postgresql_crud::StdPrimitiveF64,
                    //     //                 // postgresql_crud::StdStringString,
                    //     //                 // postgresql_crud::StdVecVecStdPrimitiveU8,
                    //     //                 // postgresql_crud::SqlxPostgresTypesPgInterval,
                    //     //                 // postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI64,
                    //     //                 // postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI32,
                    //     //                 // postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
                    //     //                 // postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTime,
                    //     //                 // //todo check all types and type decl order
                    //     //                 // postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
                    //     //                 // postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTime,
                    //     //                 // postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDate,
                    //     //                 // postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeDate,
                    //     //                 // postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesBigDecimal,
                    //     //                 // postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesDecimal,
                    //     //                 // postgresql_crud::SqlxPostgresTypesPgMoney,
                    //     //                 // postgresql_crud::SqlxPostgresTypesPgCiText,
                    //     //                 // postgresql_crud::SqlxTypesBigDecimal,
                    //     //                 // postgresql_crud::SqlxTypesDecimal,
                    //     //                 // //todo
                    //     //                 // postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
                    //     //                 // postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
                    //     //                 // postgresql_crud::SqlxTypesChronoNaiveDateTime,
                    //     //                 // postgresql_crud::SqlxTypesChronoNaiveDate,
                    //     //                 // postgresql_crud::SqlxTypesChronoNaiveTime,
                    //     //                 // postgresql_crud::SqlxPostgresTypesPgTimeTz,
                    //     //                 // postgresql_crud::SqlxTypesTimePrimitiveDateTime,
                    //     //                 // postgresql_crud::SqlxTypesTimeOffsetDateTime,
                    //     //                 // postgresql_crud::SqlxTypesTimeDate,
                    //     //                 // postgresql_crud::SqlxTypesTimeTime,
                    //     //                 // postgresql_crud::SqlxTypesUuidUuid,
                    //     //                 // postgresql_crud::SqlxTypesIpnetworkIpNetwork,
                    //     //                 // postgresql_crud::StdNetIpAddr,
                    //     //                 // postgresql_crud::SqlxTypesMacAddressMacAddress,
                    //     //                 // postgresql_crud::SqlxTypesBitVec,
                    //     //                 // // postgresql_crud::SqlxTypesJson,//todo what to do with generics?
                    //     //                 // postgresql_crud::SerdeJsonValue,
                    //     //             )
                    //     //     ),
                    //     //     modifiers(&SecurityAddon),
                    //     //     tags((name = "server", description = "server api"))
                    //     // )] //todo - this thing actually using builder pattern. maybe generate builder in GeneratePostgresqlTable then merge it together?
                    //     struct ApiDoc;
                    //     struct SecurityAddon;
                    //     impl utoipa::Modify for SecurityAddon {
                    //         fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
                    //             if let Some(components) = openapi.components.as_mut() {
                    //                 components.add_security_scheme("api_key", utoipa::openapi::security::SecurityScheme::ApiKey(utoipa::openapi::security::ApiKey::Header(utoipa::openapi::security::ApiKeyValue::new("todo_apikey"))));
                    //             }
                    //         }
                    //     }
                    //     <ApiDoc as utoipa::OpenApi>::openapi()
                    // }))
                    .into_make_service(),
            )
            .await
            .expect("2dc4449b-ece6-4a09-afd5-5ba9766f7653");
        });
}
