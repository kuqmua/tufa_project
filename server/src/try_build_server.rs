#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        common::server::routes::git_info::git_info,
        common::repositories_types::server::routes::api::cats::create_many,
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

            // common::repositories_types::server::routes::api::cats::TryCreateManyResponseVariantsTvfrr201Created,
            // common::repositories_types::server::routes::api::cats::TryCreateManyResponseVariantsTvfrr500InternalServerError,
            // common::repositories_types::server::routes::api::cats::TryCreateManyResponseVariantsTvfrr404NotFound,
            // common::repositories_types::server::routes::api::cats::TryCreateManyResponseVariantsTvfrr400BadRequest,
            // common::repositories_types::server::routes::api::cats::TryCreateManyResponseVariantsTvfrr408RequestTimeout,

            // common::repositories_types::server::routes::api::cats::TryCreateOneResponseVariantsTvfrr201Created,
            // common::repositories_types::server::routes::api::cats::TryCreateOneResponseVariantsTvfrr500InternalServerError,
            // common::repositories_types::server::routes::api::cats::TryCreateOneResponseVariantsTvfrr404NotFound,
            // common::repositories_types::server::routes::api::cats::TryCreateOneResponseVariantsTvfrr400BadRequest,
            // common::repositories_types::server::routes::api::cats::TryCreateOneResponseVariantsTvfrr408RequestTimeout,

            // common::repositories_types::server::routes::api::cats::TryReadManyResponseVariantsTvfrr200Ok,
            // common::repositories_types::server::routes::api::cats::TryReadManyResponseVariantsTvfrr500InternalServerError,
            // common::repositories_types::server::routes::api::cats::TryReadManyResponseVariantsTvfrr404NotFound,
            // common::repositories_types::server::routes::api::cats::TryReadManyResponseVariantsTvfrr400BadRequest,
            // common::repositories_types::server::routes::api::cats::TryReadManyResponseVariantsTvfrr408RequestTimeout,

            // common::repositories_types::server::routes::api::cats::TryReadOneResponseVariantsTvfrr200Ok,
            // common::repositories_types::server::routes::api::cats::TryReadOneResponseVariantsTvfrr500InternalServerError,
            // common::repositories_types::server::routes::api::cats::TryReadOneResponseVariantsTvfrr404NotFound,
            // common::repositories_types::server::routes::api::cats::TryReadOneResponseVariantsTvfrr400BadRequest,
            // common::repositories_types::server::routes::api::cats::TryReadOneResponseVariantsTvfrr408RequestTimeout,

            // common::repositories_types::server::routes::api::cats::TryUpdateManyResponseVariantsTvfrr200Ok,
            // common::repositories_types::server::routes::api::cats::TryUpdateManyResponseVariantsTvfrr500InternalServerError,
            // common::repositories_types::server::routes::api::cats::TryUpdateManyResponseVariantsTvfrr404NotFound,
            // common::repositories_types::server::routes::api::cats::TryUpdateManyResponseVariantsTvfrr400BadRequest,
            // common::repositories_types::server::routes::api::cats::TryUpdateManyResponseVariantsTvfrr408RequestTimeout,

            // common::repositories_types::server::routes::api::cats::TryUpdateOneResponseVariantsTvfrr200Ok,
            // common::repositories_types::server::routes::api::cats::TryUpdateOneResponseVariantsTvfrr500InternalServerError,
            // common::repositories_types::server::routes::api::cats::TryUpdateOneResponseVariantsTvfrr404NotFound,
            // common::repositories_types::server::routes::api::cats::TryUpdateOneResponseVariantsTvfrr400BadRequest,
            // common::repositories_types::server::routes::api::cats::TryUpdateOneResponseVariantsTvfrr408RequestTimeout,

            // common::repositories_types::server::routes::api::cats::TryDeleteManyResponseVariantsTvfrr200Ok,
            // common::repositories_types::server::routes::api::cats::TryDeleteManyResponseVariantsTvfrr500InternalServerError,
            // common::repositories_types::server::routes::api::cats::TryDeleteManyResponseVariantsTvfrr404NotFound,
            // common::repositories_types::server::routes::api::cats::TryDeleteManyResponseVariantsTvfrr400BadRequest,
            // common::repositories_types::server::routes::api::cats::TryDeleteManyResponseVariantsTvfrr408RequestTimeout,

            // common::repositories_types::server::routes::api::cats::TryDeleteOneResponseVariantsTvfrr200Ok,
            // common::repositories_types::server::routes::api::cats::TryDeleteOneResponseVariantsTvfrr500InternalServerError,
            // common::repositories_types::server::routes::api::cats::TryDeleteOneResponseVariantsTvfrr404NotFound,
            // common::repositories_types::server::routes::api::cats::TryDeleteOneResponseVariantsTvfrr400BadRequest,
            // common::repositories_types::server::routes::api::cats::TryDeleteOneResponseVariantsTvfrr408RequestTimeout,

            // common::repositories_types::server::routes::api::cats::CreateManyPayload,
            // common::repositories_types::server::routes::api::cats::CreateManyPayloadElement,

            // common::repositories_types::server::routes::api::cats::CreateOnePayload,

            common::common::utoipa::std::time::StdTimeDuration,
            // common::server::postgres::uuid_wrapper::PossibleUuidWrapper,
            error_occurence_lib::code_occurence::CodeOccurence,
            //
            postgresql_crud::TimeMonthWithSerializeDeserialize,
            postgresql_crud::SqlxTypesTimeUtcOffsetFromHmsWithSerializeDeserialize,
            postgresql_crud::NumBigintSignWithSerializeDeserialize,
            postgresql_crud::NumBigintBigIntWithSerializeDeserialize,
            postgresql_crud::StdPrimitiveBoolWithSerializeDeserialize,
            postgresql_crud::StdPrimitiveI16WithSerializeDeserialize,
            postgresql_crud::StdPrimitiveI32WithSerializeDeserialize,
            postgresql_crud::StdPrimitiveI64WithSerializeDeserialize,
            postgresql_crud::StdPrimitiveF32WithSerializeDeserialize,
            postgresql_crud::StdPrimitiveF64WithSerializeDeserialize,
            postgresql_crud::StdStringStringWithSerializeDeserialize,
            postgresql_crud::StdVecVecStdPrimitiveU8WithSerializeDeserialize,
            postgresql_crud::SqlxPostgresTypesPgIntervalWithSerializeDeserialize,
            postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI64WithSerializeDeserialize,
            postgresql_crud::SqlxPostgresTypesPgRangeStdPrimitiveI32WithSerializeDeserialize,
            postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize,
            postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize,
            //todo check all types and type decl order
            postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize,
            postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeOffsetDateTimeWithSerializeDeserialize,
            postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateWithSerializeDeserialize,
            postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesTimeDateWithSerializeDeserialize,
            postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalWithSerializeDeserialize,
            postgresql_crud::SqlxPostgresTypesPgRangeSqlxTypesDecimalWithSerializeDeserialize,
            postgresql_crud::SqlxPostgresTypesPgMoneyWithSerializeDeserialize,
            postgresql_crud::SqlxPostgresTypesPgCiTextWithSerializeDeserialize,
            postgresql_crud::SqlxTypesBigDecimalWithSerializeDeserialize,
            postgresql_crud::SqlxTypesDecimalWithSerializeDeserialize,
            //todo
            postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoLocalWithSerializeDeserialize,
            postgresql_crud::SqlxTypesChronoDateTimeSqlxTypesChronoUtcWithSerializeDeserialize,
            postgresql_crud::SqlxTypesChronoNaiveDateTimeWithSerializeDeserialize,
            postgresql_crud::SqlxTypesChronoNaiveDateWithSerializeDeserialize,
            postgresql_crud::SqlxTypesChronoNaiveTimeWithSerializeDeserialize,
            postgresql_crud::SqlxPostgresTypesPgTimeTzWithSerializeDeserialize,
            postgresql_crud::SqlxTypesTimePrimitiveDateTimeWithSerializeDeserialize,
            postgresql_crud::SqlxTypesTimeOffsetDateTimeWithSerializeDeserialize,
            postgresql_crud::SqlxTypesTimeDateWithSerializeDeserialize,
            postgresql_crud::SqlxTypesTimeTimeWithSerializeDeserialize,
            postgresql_crud::SqlxTypesUuidUuidWithSerializeDeserialize,
            postgresql_crud::SqlxTypesIpnetworkIpNetworkWithSerializeDeserialize,
            postgresql_crud::StdNetIpAddrWithSerializeDeserialize,
            postgresql_crud::SqlxTypesMacAddressMacAddressWithSerializeDeserialize,
            postgresql_crud::SqlxTypesBitVecWithSerializeDeserialize,
            // postgresql_crud::SqlxTypesJsonWithSerializeDeserialize,//todo what to do with generics?
            postgresql_crud::SerdeJsonValueWithSerializeDeserialize,
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
            components.add_security_scheme(
                "api_key",
                utoipa::openapi::security::SecurityScheme::ApiKey(
                    utoipa::openapi::security::ApiKey::Header(
                        utoipa::openapi::security::ApiKeyValue::new("todo_apikey"),
                    ),
                ),
            )
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
    pub message: std::string::String,
}

#[derive(Clone)] //or maybe add Clone to AppInfo too to solve possible problem?
struct HeaderMessage(pub std::string::String);

async fn read_middleware_custom_header(
    axum::Extension(message): axum::Extension<HeaderMessage>,
) -> std::string::String {
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
pub async fn try_build_server<'a>(
    postgres_pool: sqlx::Pool<sqlx::Postgres>,
    config: &'static common::repositories_types::server::config::config_struct::Config,
) -> Result<(), Box<common::repositories_types::server::try_build_server::TryBuildServer>> {
    println!(
        "server running on {}",
        common::common::config::get_server_address::GetServerAddress::get_server_address(&config)
    );
    let app_state = std::sync::Arc::new(
        common::repositories_types::server::routes::app_state::AppState {
            postgres_pool,
            config,
            project_git_info: &git_info::PROJECT_GIT_INFO,
        },
    );
    let shared_data = SharedData {
        message: std::string::String::from("shared_message"),
    };
    axum::serve(
        tokio::net::TcpListener::bind(config_lib::config_fields::GetSocketAddr::get_socket_addr(
            config,
        ))
        .await
        .unwrap(),
        // common::common::config::config_fields::GetSocketAddr::get_socket_addr(config),
        axum::Router::new()
            .route(
                "/read_middleware_custom_header",
                axum::routing::get(read_middleware_custom_header),
            )
            // .route(
            //     "/header_extractor_example",
            //     axum::routing::get(header_extractor_example),
            // )
            // .route(
            //     "/extract_custom_header_example",
            //     axum::routing::get(extract_custom_header_example),
            // )
            // .route_layer(axum::middleware::from_fn(set_middleware_custom_header))
            .route(
                "/middleware_message_example",
                axum::routing::get(middleware_message_example),
            )
            .layer(axum::Extension(shared_data))
            .merge(common::server::routes::routes(app_state.clone()))
            .merge(crate::routes::api::routes(app_state.clone()))
            .merge(common::server::routes::not_found::not_found_route(
                app_state.clone(),
            ))
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
            .merge(
                utoipa_swagger_ui::SwaggerUi::new(
                    common::global_variables::hardcode::SLASH_SWAGGER_UI,
                )
                .url("/api-docs/openapi.json", {
                    use utoipa::OpenApi;
                    ApiDoc::openapi()
                }),
            )
            .into_make_service(),
    )
    .await
    .unwrap_or_else(|e| panic!("axum builder serve await failed {e:#?}"));
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
