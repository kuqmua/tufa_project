// use crate::fetch::rss_metainfo_fetch_structures::NoItemsError;
// use crate::helpers::write_json_into_file::write_json_into_file;
// use chrono::Local;
// use futures::future::join_all;
// use serde_json::json;
// use std::time::Instant;
// use crate::lazy_static::config::CONFIG;

// #[deny(
//     clippy::indexing_slicing,
//     clippy::unwrap_used,
//     clippy::integer_arithmetic,
//     clippy::float_arithmetic
// )]
// #[tokio::main]
// pub async fn rss_async_write_fetch_error_logs_into_files_wrapper(
//     error_posts: Vec<PostErrorVariant>,
// ) {
//     let time = Instant::now();
//     let _ = join_all(error_posts.into_iter().map(|post_error_variant| async {
//         match post_error_variant {
//             PostErrorVariant::NoItems {
//                 link,
//                 no_items_error,
//                 provider_kind: pk,
//             } => {
//                 let replaced_link = link.replace('/', "-").replace(':', "-").replace('.', "-");
//                 let json_object = NoItemsError::into_json_with_link_and_provider_kind(
//                     &link,
//                     &no_items_error,
//                     &pk,
//                 );
//                 let path_to_file_string = format!(
//                     "logs/{}/{pk}/{}/{pk}-{replaced_link}.json",
//                     &CONFIG.warning_logs_directory_name,
//                     &CONFIG.unhandled_success_handled_success_are_there_items_initialized_posts_dir
//                 );
//                 let path_to_file = std::path::Path::new(&path_to_file_string);
//                 (pk, write_json_into_file(path_to_file, json_object).await)
//             }
//             PostErrorVariant::RssFetchAndParseProviderDataError {
//                 link,
//                 provider_kind: pk,
//                 error,
//             } => {
//                 let replaced_link = link.replace("/", "-").replace(":", "-").replace(".", "-");
//                 let path_to_file_string = format!(
//                     "logs/{}/{pk}/{}/{pk}-{replaced_link}.json",
//                     &CONFIG.warning_logs_directory_name,
//                     &CONFIG.unhandled_success_handled_success_are_there_items_initialized_posts_dir
//                 );
//                 let path_to_file = std::path::Path::new(&path_to_file_string);
//                 let json_object = json!({
//                     "link": link,
//                     "stringified_error": error.to_string(),
//                     "part_of": format!("{pk}"),
//                     "date": Local::now().to_string()
//                 });
//                 (pk, write_json_into_file(path_to_file, json_object).await)
//             }
//         }
//     }))
//     .await; //todo: add state of success/unsuccess
//     if CONFIG.is_time_measurement_prints_enabled {
//         println!(
//             "write fetch error logs into files done in {} seconds {} miliseconds",
//             time.elapsed().as_secs(),
//             time.elapsed().as_millis(),
//         );
//     }
// }
