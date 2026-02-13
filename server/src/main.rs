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
            let pg_pool = PgPoolOptions::new()
                .max_connections(50)
                .connect(ExposeSecret::expose_secret(
                    GetDatabaseUrl::get_database_url(&config),
                ))
                .await
                .expect("8b72f688-be7d-4f5c-9185-44a27290a9d0");
            TableExample::prepare_pg(&pg_pool)
                .await
                .expect("647fa499-c465-432d-ba4a-498f3e943ada");
            let tcp_listener =
                TcpListener::bind(GetServiceSocketAddress::get_service_socket_address(&config))
                    .await
                    .expect("3f294e7c-3386-497f-b76c-c0364d59a60d");
            let app_state = Arc::new(ServerAppState {
                pg_pool,
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
                    //todo partialy move to gen postresql crud implementation (except git_info route)
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
                    //     //                 // pg_crud::TimeMonth,
                    //     //                 // pg_crud::SqlxTypesTimeUtcOffset,
                    //     //                 // pg_crud::NumBigintSign,
                    //     //                 // pg_crud::NumBigintBigInt,
                    //     //                 // pg_crud::StdPrimitiveBool,
                    //     //                 // pg_crud::StdPrimitiveI16,
                    //     //                 // pg_crud::StdPrimitiveI32,
                    //     //                 // pg_crud::StdPrimitiveI64,
                    //     //                 // pg_crud::StdPrimitiveF32,
                    //     //                 // pg_crud::StdPrimitiveF64,
                    //     //                 // pg_crud::StdStringString,
                    //     //                 // pg_crud::StdVecVecStdPrimitiveU8,
                    //     //                 // pg_crud::SqlxPgTypesPgInterval,
                    //     //                 // pg_crud::SqlxPgTypesPgRangeStdPrimitiveI64,
                    //     //                 // pg_crud::SqlxPgTypesPgRangeStdPrimitiveI32,
                    //     //                 // pg_crud::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
                    //     //                 // pg_crud::SqlxPgTypesPgRangeSqlxTypesTimePrimitiveDateTime,
                    //     //                 // //todo check all types and type decl order
                    //     //                 // pg_crud::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocal,
                    //     //                 // pg_crud::SqlxPgTypesPgRangeSqlxTypesTimeOffsetDateTime,
                    //     //                 // pg_crud::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDate,
                    //     //                 // pg_crud::SqlxPgTypesPgRangeSqlxTypesTimeDate,
                    //     //                 // pg_crud::SqlxPgTypesPgRangeSqlxTypesBigDecimal,
                    //     //                 // pg_crud::SqlxPgTypesPgRangeSqlxTypesDecimal,
                    //     //                 // pg_crud::SqlxPgTypesPgMoney,
                    //     //                 // pg_crud::SqlxPgTypesPgCiText,
                    //     //                 // pg_crud::SqlxTypesBigDecimal,
                    //     //                 // pg_crud::SqlxTypesDecimal,
                    //     //                 // //todo
                    //     //                 // pg_crud::SqlxTypesChronoDateTimeSqlxTypesChronoLocal,
                    //     //                 // pg_crud::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
                    //     //                 // pg_crud::SqlxTypesChronoNaiveDateTime,
                    //     //                 // pg_crud::SqlxTypesChronoNaiveDate,
                    //     //                 // pg_crud::SqlxTypesChronoNaiveTime,
                    //     //                 // pg_crud::SqlxPgTypesPgTimeTz,
                    //     //                 // pg_crud::SqlxTypesTimePrimitiveDateTime,
                    //     //                 // pg_crud::SqlxTypesTimeOffsetDateTime,
                    //     //                 // pg_crud::SqlxTypesTimeDate,
                    //     //                 // pg_crud::SqlxTypesTimeTime,
                    //     //                 // pg_crud::SqlxTypesUuidUuid,
                    //     //                 // pg_crud::SqlxTypesIpnetworkIpNetwork,
                    //     //                 // pg_crud::StdNetIpAddr,
                    //     //                 // pg_crud::SqlxTypesMacAddressMacAddress,
                    //     //                 // pg_crud::SqlxTypesBitVec,
                    //     //                 // // pg_crud::SqlxTypesJson,//todo what to do with generics?
                    //     //                 // pg_crud::SerdeJsonValue,
                    //     //             )
                    //     //     ),
                    //     //     modifiers(&SecurityAddon),
                    //     //     tags((name = "server", description = "server api"))
                    //     // )] //todo - this thing actually using builder pattern. maybe gen builder in GenPgTable then merge it together?
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
