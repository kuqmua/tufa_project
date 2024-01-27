#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        tufa_common::server::routes::git_info::git_info,
        
        tufa_common::repositories_types::tufa_server::routes::api::cats::create_many,
        tufa_common::repositories_types::tufa_server::routes::api::cats::create_one,
        tufa_common::repositories_types::tufa_server::routes::api::cats::read_many,
        tufa_common::repositories_types::tufa_server::routes::api::cats::read_one,
        tufa_common::repositories_types::tufa_server::routes::api::cats::update_many,
        tufa_common::repositories_types::tufa_server::routes::api::cats::update_one,
        tufa_common::repositories_types::tufa_server::routes::api::cats::delete_many,
        tufa_common::repositories_types::tufa_server::routes::api::cats::delete_one
    ),
    components(
        schemas(
            tufa_common::server::routes::git_info::GitInfo,

            tufa_common::repositories_types::tufa_server::routes::api::cats::TryCreateManyResponseVariantsTvfrr201Created,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryCreateManyResponseVariantsTvfrr500InternalServerError,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryCreateManyResponseVariantsTvfrr404NotFound,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryCreateManyResponseVariantsTvfrr400BadRequest,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryCreateManyResponseVariantsTvfrr408RequestTimeout,

            tufa_common::repositories_types::tufa_server::routes::api::cats::TryCreateOneResponseVariantsTvfrr201Created,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryCreateOneResponseVariantsTvfrr500InternalServerError,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryCreateOneResponseVariantsTvfrr404NotFound,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryCreateOneResponseVariantsTvfrr400BadRequest,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryCreateOneResponseVariantsTvfrr408RequestTimeout,

            tufa_common::repositories_types::tufa_server::routes::api::cats::TryReadManyResponseVariantsTvfrr200Ok,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryReadManyResponseVariantsTvfrr500InternalServerError,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryReadManyResponseVariantsTvfrr404NotFound,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryReadManyResponseVariantsTvfrr400BadRequest,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryReadManyResponseVariantsTvfrr408RequestTimeout,

            tufa_common::repositories_types::tufa_server::routes::api::cats::TryReadOneResponseVariantsTvfrr200Ok,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryReadOneResponseVariantsTvfrr500InternalServerError,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryReadOneResponseVariantsTvfrr404NotFound,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryReadOneResponseVariantsTvfrr400BadRequest,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryReadOneResponseVariantsTvfrr408RequestTimeout,

            tufa_common::repositories_types::tufa_server::routes::api::cats::TryUpdateManyResponseVariantsTvfrr200Ok,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryUpdateManyResponseVariantsTvfrr500InternalServerError,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryUpdateManyResponseVariantsTvfrr404NotFound,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryUpdateManyResponseVariantsTvfrr400BadRequest,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryUpdateManyResponseVariantsTvfrr408RequestTimeout,

            tufa_common::repositories_types::tufa_server::routes::api::cats::TryUpdateOneResponseVariantsTvfrr200Ok,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryUpdateOneResponseVariantsTvfrr500InternalServerError,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryUpdateOneResponseVariantsTvfrr404NotFound,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryUpdateOneResponseVariantsTvfrr400BadRequest,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryUpdateOneResponseVariantsTvfrr408RequestTimeout,

            tufa_common::repositories_types::tufa_server::routes::api::cats::TryDeleteManyResponseVariantsTvfrr200Ok,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryDeleteManyResponseVariantsTvfrr500InternalServerError,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryDeleteManyResponseVariantsTvfrr404NotFound,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryDeleteManyResponseVariantsTvfrr400BadRequest,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryDeleteManyResponseVariantsTvfrr408RequestTimeout,

            tufa_common::repositories_types::tufa_server::routes::api::cats::TryDeleteOneResponseVariantsTvfrr200Ok,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryDeleteOneResponseVariantsTvfrr500InternalServerError,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryDeleteOneResponseVariantsTvfrr404NotFound,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryDeleteOneResponseVariantsTvfrr400BadRequest,
            tufa_common::repositories_types::tufa_server::routes::api::cats::TryDeleteOneResponseVariantsTvfrr408RequestTimeout,

            tufa_common::repositories_types::tufa_server::routes::api::cats::CreateManyPayload,
            tufa_common::repositories_types::tufa_server::routes::api::cats::CreateManyPayloadElement,

            tufa_common::repositories_types::tufa_server::routes::api::cats::CreateOnePayload,

            tufa_common::common::utoipa::std::time::StdTimeDuration,
            error_occurence_lib::git_info::GitInfoWithoutLifetime,
            tufa_common::server::postgres::uuid_wrapper::PossibleUuidWrapper,
            tufa_common::common::code_occurence::CodeOccurence,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "tufa_server", description = "tufa_server api")
    )
)]//todo - this thing actually using builder pattern. maybe generate builder in GeneratePostgresqlCrud then merge it together?
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
//     let pc = headers.get("project_commit");
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
//     let request_project_commit = req
//         .headers()
//         .get(tufa_common::common::git::project_git_info::PROJECT_COMMIT)
//         .ok_or_else(|| axum::http::StatusCode::BAD_REQUEST)?;
//     let project_commit_checker_header = request_project_commit
//         .to_str()
//         .map_err(|_error| axum::http::StatusCode::BAD_REQUEST)?
//         .to_owned();
//     let extensions = req.extensions_mut();
//     extensions.insert(HeaderMessage(project_commit_checker_header.to_owned()));
//     Ok(next.run(req).await)
// }

//todo - make it async trait after async trait stabilization
pub async fn try_build_server<'a>(
    postgres_pool: sqlx::Pool<sqlx::Postgres>,
    config: &'static tufa_common::repositories_types::tufa_server::config::config_struct::Config,
) -> Result<(), Box<tufa_common::repositories_types::tufa_server::try_build_server::TryBuildServer>>
{
    println!(
        "server running on {}",
        tufa_common::common::config::get_server_address::GetServerAddress::get_server_address(
            &config
        )
    );
    let app_info = std::sync::Arc::new(
        tufa_common::repositories_types::tufa_server::routes::app_info::AppInfo {
            postgres_pool,
            config,
            project_git_info:
                &tufa_common::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO,
            repository_git_info: &crate::global_variables::compile_time::git_info::GIT_INFO,
        },
    ); //: std::sync::Arc<dyn tufa_common::repositories_types::tufa_server::routes::service_possibilities::ServicePossibilities + Send + Sync>
    let shared_data = SharedData {
        message: std::string::String::from("shared_message"),
    };
    axum::serve(
         tokio::net::TcpListener::bind(tufa_common::common::config::config_fields::GetSocketAddr::get_socket_addr(config)).await.unwrap(),
        // tufa_common::common::config::config_fields::GetSocketAddr::get_socket_addr(config),
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
            .merge(tufa_common::server::routes::routes(app_info.clone()))
            .merge(crate::routes::api::routes(app_info.clone()))
            .merge(tufa_common::server::routes::not_found::not_found_route(
                app_info.clone(),
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
                utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", {
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
//     app_info_state: axum::extract::State<std::sync::Arc<dyn GetOneGetTwo + Send + Sync>>,
// ) {
//     let example = Example {};
//     example.do_something(app_info_state.as_ref());
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
//     let app_info: std::sync::Arc<dyn GetOneGetTwo + Send + Sync> =
//         std::sync::Arc::new(AppInfo(One(String::from("one")), Two(String::from("two"))));
//     let router = axum::Router::new()
//         .route("/", axum::routing::get(something))
//         // expected struct `std::boxed::Box<std::sync::Arc<(dyn GetOneGetTwo + std::marker::Send + std::marker::Sync + 'static)>>`
//         // found struct `std::boxed::Box<std::sync::Arc<AppInfo>>`
//         .with_state(app_info)
//         .into_make_service();
//     //
//     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//         .serve(router)
//         .await
//         .unwrap();
// }
