// use crate::fetch::rss_async_write_fetch_error_logs_into_files_wrapper::rss_async_write_fetch_error_logs_into_files_wrapper;
// use crate::logs_logic::async_write_fetch_error_logs_into_mongo_wrapper::async_write_fetch_error_logs_into_mongo_wrapper;
// use crate::logs_logic::async_write_fetch_error_logs_into_mongo_wrapper::WriteLogsResult;
// use crate::prints::print_colorful_message::print_colorful_message;
// use crate::prints::print_type_enum::PrintType;
// use crate::lazy_static::config::CONFIG;
// use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
// use crate::traits::provider_kind_trait::ProviderKindTrait;

// #[deny(
//     clippy::indexing_slicing,
//     clippy::unwrap_used,
//     clippy::integer_arithmetic,
//     clippy::float_arithmetic
// )]
// pub async fn write_error_posts_wrapper(error_posts: Vec<PostErrorVariant>) {
//     //todo add flag in config or if its already exists put it here
//     //maybe instead of if write match to local or to mongo or postgres
//     if CONFIG.is_write_error_logs_in_local_folder_enabled {
//         let cleaning_hashmap_result = ProviderKind::remove_existing_providers_logs_directories();
//         //todo add enable_writing logs if not clean or not enabled cleaning
//         match cleaning_hashmap_result {
//             Ok(()) => {
//                 rss_async_write_fetch_error_logs_into_files_wrapper(error_posts);
//             }
//             Err(error_hashmap) => {
//                 for (pk, error) in error_hashmap {
//                     print_colorful_message(
//                         Some(&pk),
//                         PrintType::Error,
//                         file!().to_string(),
//                         line!().to_string(),
//                         format!("ProviderKind::remove_providers_logs_directories() failed for {pk:#?} (todo2) error: {error:#?}"),
//                     );
//                 }
//             }
//         }
//     } else if CONFIG.is_mongo_write_error_logs_enabled {
//         //remove writing logs dublication in different sources coz need to be cloned
//         let result = async_write_fetch_error_logs_into_mongo_wrapper(error_posts).await;
//         match result {
//             WriteLogsResult::Success => {
//                 print_colorful_message(
//                     None,
//                     PrintType::Success,
//                     file!().to_string(),
//                     line!().to_string(),
//                     format!("async_write_fetch_error_logs_into_mongo_wrapper result {result:#?}"),
//                 );
//             }
//             WriteLogsResult::PartialSuccess => {
//                 print_colorful_message(
//                     None,
//                     PrintType::PartialSuccess,
//                     file!().to_string(),
//                     line!().to_string(),
//                     format!("async_write_fetch_error_logs_into_mongo_wrapper result {result:#?}"),
//                 );
//             }
//             WriteLogsResult::Failure => {
//                 print_colorful_message(
//                     None,
//                     PrintType::Error,
//                     file!().to_string(),
//                     line!().to_string(),
//                     format!("async_write_fetch_error_logs_into_mongo_wrapper result {result:#?}"),
//                 );
//             }
//         }
//     }
// }
