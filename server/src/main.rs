pub mod global_variables;
// #[cfg(test)]
// mod tests;
//query! containing mods
// pub mod authentication;
// pub mod idempotency;
// pub mod issue_delivery_worker;
pub mod routes;

fn main() {
    std::thread::Builder::new().stack_size(16 * 1024 * 1024) // 16 MB
    .spawn(|| {
        tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
        .unwrap()
        .block_on(
            async {
                println!("commit {}", git_info::PROJECT_GIT_INFO.commit);
                let config = crate::global_variables::runtime::config::CONFIG.get_or_init(|| common::repositories_types::server::config::Config::try_from_env().unwrap());
                // if let Err(error) = common::repositories_types::server::telemetry::init_subscriber::init_subscriber(common::repositories_types::server::telemetry::get_subscriber::get_subscriber(env!("CARGO_PKG_VERSION"), config, std::io::stdout)) {
                //     panic!("common::repositories_types::server::telemetry::init_subscriber::init_subscriber failed, error: {error:#?}")
                // }
                println!("trying to create postgres pool...");
                let pg_pool = sqlx::postgres::PgPoolOptions::new().connect(secrecy::ExposeSecret::expose_secret(app_state::GetDatabaseUrl::get_database_url(&config))).await.unwrap();//todo remove .unwrap()
                //todo rename create_table_if_not_exists
                println!("create_table_if_not_exists...");
                common::repositories_types::server::routes::api::example::Example::create_table_if_not_exists(&pg_pool).await.unwrap();//todo remove .unwrap()
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
                println!("trying to build server...");
                try_build_server(
                    pg_pool, // redis_session_storage,
                    config,
                ).await.unwrap();
            }
        );
    })
    .unwrap()
    .join()
    .unwrap();
}




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
    tags(
        (name = "server", description = "server api")
    )
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

// // allow to open source files in browser like php
// fn routes_static() -> axum::Router {
//     axum::Router::new().nest_service(
//         "/",
//         axum::routing::get_service(tower_http::services::ServeDir::new("./")),
//     )
// }

// async fn extract_custom_header_example(headers: http::header::HeaderMap) {
//     let pc = headers.get("commit");
//     println!("pc{pc:#?}")
// }

// async fn header_extractor_example(
//     axum::TypedHeader(header): axum::TypedHeader<axum::headers::UserAgent>,
// ) {
//     println!("header{:#?}", header);
// }

async fn middleware_message_example(axum::Extension(shared_data): axum::Extension<SharedData>) {
    println!("message {}", shared_data.message);
}

#[derive(Clone)]
struct SharedData {
    pub(crate) message: std::string::String,
}

#[derive(Clone)] //or maybe add Clone to AppInfo too to solve possible problem?
struct HeaderMessage(pub(crate) std::string::String);

async fn read_middleware_custom_header(axum::Extension(message): axum::Extension<HeaderMessage>) -> std::string::String {
    println!("read_middleware_custom_header {}", message.0);
    message.0
}

// pub async fn set_middleware_custom_header<B>(
//     mut req: axum::http::Request<B>,
//     next: axum::middleware::Next<B>,
// ) -> Result<axum::response::Response, axum::http::StatusCode> {
//     let request_commit = req
//         .headers()
//         .get("commit")//todo maybe reuse global variable
//         .ok_or_else(|| axum::http::StatusCode::BAD_REQUEST)?;
//     let commit_checker_header = request_commit
//         .to_str()
//         .map_err(|_error| axum::http::StatusCode::BAD_REQUEST)?
//         .to_owned();
//     let extensions = req.extensions_mut();
//     extensions.insert(HeaderMessage(commit_checker_header.to_owned()));
//     Ok(next.run(req).await)
// }

//todo - make it async trait after async trait stabilization
async fn try_build_server(postgres_pool: sqlx::Pool<sqlx::Postgres>, config: &'static common::repositories_types::server::config::Config) -> Result<(), Box<common::repositories_types::server::try_build_server::TryBuildServer>> {
    println!("server running on {}", app_state::GetServiceSocketAddress::get_service_socket_address(&config));
    let app_state = std::sync::Arc::new(common::repositories_types::server::routes::app_state::AppState {
        postgres_pool,
        config,
        project_git_info: &git_info::PROJECT_GIT_INFO,
    });
    let shared_data = SharedData { message: std::string::String::from("shared_message") };
    axum::serve(
        tokio::net::TcpListener::bind(app_state::GetServiceSocketAddress::get_service_socket_address(config)).await.unwrap(),
        axum::Router::new()
            .route("/read_middleware_custom_header", axum::routing::get(read_middleware_custom_header))
            // .route(
            //     "/header_extractor_example",
            //     axum::routing::get(header_extractor_example),
            // )
            // .route(
            //     "/extract_custom_header_example",
            //     axum::routing::get(extract_custom_header_example),
            // )
            // .route_layer(axum::middleware::from_fn(set_middleware_custom_header))
            .route("/middleware_message_example", axum::routing::get(middleware_message_example))
            .layer(axum::Extension(shared_data))
            .merge(common::server::routes::routes(std::sync::Arc::<common::repositories_types::server::routes::app_state::AppState<'_>>::clone(&app_state)))
            .merge(crate::routes::api::routes(std::sync::Arc::<common::repositories_types::server::routes::app_state::AppState<'_>>::clone(&app_state)))
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
                use utoipa::OpenApi;
                ApiDoc::openapi()
            }))
            .into_make_service(),
    )
    .await
    .unwrap_or_else(|error| panic!("axum builder serve await failed {error:#?}"));
    Ok(())
}

///////////////////////////////////
// this     works

// pub struct One(String);
// pub struct Two(String);
// pub struct AppInfo(One, Two);
// pub trait GetOne {
//     fn get_one(&self) -> &One;
// }
// pub trait GetTwo {
//     fn get_two(&self) -> &Two;
// }
// pub trait GetOneGetTwo: GetOne + GetTwo {}
// impl GetOne for AppInfo {
//     fn get_one(&self) -> &One {
//         &self.0
//     }
// }
// impl GetTwo for AppInfo {
//     fn get_two(&self) -> &Two {
//         &self.1
//     }
// }
// impl GetOneGetTwo for AppInfo {}
// pub struct Example {}
// pub trait DoSomething {
//     fn do_something<T: GetOne + ?Sized>(&self, handle_get_one: &T);
// }
// impl DoSomething for Example {
//     fn do_something<T: GetOne + ?Sized>(&self, handle_get_one: &T) {
//         println!("{}", handle_get_one.get_one().0);
//     }
// }
// pub async fn something(
//     app_state: axum::extract::State<std::sync::Arc<dyn GetOneGetTwo + Send + Sync>>,
// ) {
//     let example = Example {};
//     example.do_something(app_state.as_ref());
// }
// #[tokio::main]
// async fn main() {
//     //case1 - compiles
//     // let router = axum::Router::new()
//     //     .route("/", axum::routing::get(something))
//     //     .with_state(Box::new(std::sync::Arc::new(AppInfo(
//     //         One(String::from("one")),
//     //         Two(String::from("two")),
//     //     ))))
//     //     .into_make_service();
//     //case2
//     let app_state: std::sync::Arc<dyn GetOneGetTwo + Send + Sync> =
//         std::sync::Arc::new(AppInfo(One(String::from("one")), Two(String::from("two"))));
//     let router = axum::Router::new()
//         .route("/", axum::routing::get(something))
//         // expected struct `std::boxed::Box<std::sync::Arc<(dyn GetOneGetTwo + std::marker::Send + std::marker::Sync + 'static)>>`
//         // found struct `std::boxed::Box<std::sync::Arc<AppInfo>>`
//         .with_state(app_state)
//         .into_make_service();
//     //
//     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//         .serve(router)
//         .await
//         .unwrap();
// }
