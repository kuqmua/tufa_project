use app_state::{GetDatabaseUrl, GetServiceSocketAddress};
use axum::{Router, serve};
use common_routes::common_routes;
use git_info::PROJECT_GIT_INFO;
use num_cpus::get;
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
        .worker_threads(get())
        .enable_all()
        .build()
        .expect("5995c954")
        .block_on(async {
            let config = Config::try_from_env().expect("d74a6e5f");
            let pg_pool = PgPoolOptions::new()
                .max_connections(50)
                .connect(ExposeSecret::expose_secret(
                    GetDatabaseUrl::get_database_url(&config),
                ))
                .await
                .expect("8b72f688");
            TableExample::prepare_pg(&pg_pool).await.expect("647fa499");
            let tcp_listener =
                TcpListener::bind(GetServiceSocketAddress::get_service_socket_address(&config))
                    .await
                    .expect("3f294e7c");
            let app_state = Arc::new(ServerAppState {
                pg_pool,
                config,
                project_git_info: &PROJECT_GIT_INFO,
            });
            serve(
                tcp_listener,
                Router::new()
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
                            .allow_origin(["http://127.0.0.1".parse().expect("2a0b7c30")]),
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
                    //     //                 // pg_crud::Bool,
                    //     //                 // pg_crud::I16,
                    //     //                 // pg_crud::I32,
                    //     //                 // pg_crud::I64,
                    //     //                 // pg_crud::F32,
                    //     //                 // pg_crud::F64,
                    //     //                 // pg_crud::StdStringString,
                    //     //                 // pg_crud::StdVecVecU8,
                    //     //                 // pg_crud::SqlxPgTypesPgInterval,
                    //     //                 // pg_crud::SqlxPgTypesPgRangeI64,
                    //     //                 // pg_crud::SqlxPgTypesPgRangeI32,
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
            .expect("2dc4449b");
        });
}
