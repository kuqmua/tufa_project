use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::preparation::prepare_server::prepare_server;
use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;
use crate::project_constants::PROJECT_NAME;
use crate::server_wrapper::server_wrapper;
use crate::telemetry::get_subscriber::get_subscriber;
use crate::telemetry::init_subscriber::init_subscriber;
// use valuable::Valuable;

// #[derive(Clone, Debug, Valuable)]
// struct User {
//     name: String,
//     age: u32,
//     something: Vec<bool>,
//     address: Address,
// }

// #[derive(Clone, Debug, Valuable)]
// struct Address {
//     country: String,
//     city: String,
//     street: String,
// }

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn entry() {
    match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
    {
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::Error,
                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                format!("Cannot build tokio runtime {e:#?}"),
            );
        }
        Ok(runtime) => {
            if let true = CONFIG.is_tracing_enabled {
                if let Err(e) = init_subscriber(get_subscriber(
                    PROJECT_NAME.into(),
                    CONFIG.tracing_type.to_lower_snake_case(),
                    std::io::stdout,
                )) {
                    print_colorful_message(
                        None,
                        PrintType::Error,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        format!("tracing init_subscriber error: {:#?}", e),
                    );
                    return;
                };
            }
            // let user = User {
            //     name: "Arwen Undomiel".to_string(),
            //     age: 3000,
            //     something: vec![true, false],
            //     address: Address {
            //         country: "Middle Earth".to_string(),
            //         city: "Rivendell".to_string(),
            //         street: "leafy lane".to_string(),
            //     },
            // };
            // tracing::error!(valuable = false, user = ?user);
            if let true = CONFIG.is_preparation_enabled {
                if runtime.block_on(prepare_server(true)).is_err() {
                    return;
                }
            }
            if let Err(e) = server_wrapper() {
                print_colorful_message(
                    None,
                    PrintType::Error,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                    format!("Cannot run actix-web HttpServer, error: {:#?}", e),
                );
            }
        }
    }
}
