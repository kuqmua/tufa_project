// use crate::fetch::rss_metainfo_fetch_structures::NoItemsError;
// use crate::logs_logic::drop_mongo_provider_logs_collection_if_need::drop_mongo_provider_logs_collection_if_need;
// use crate::lazy_static::config::CONFIG;
// use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
// use crate::mongo_integration::mongo_drop_db::mongo_drop_db;
// use crate::mongo_integration::mongo_get_db_url::mongo_get_db_url;
// use crate::mongo_integration::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection;
// use std::time::Instant;
// use crate::prints::print_colorful_message::print_colorful_message;
// use crate::prints::print_type_enum::PrintType;
// use std::collections::HashMap;
// use chrono::Local;
// use futures::future::join_all;
// use serde_json::json;

// #[derive(
//     Clone, Debug, serde_derive::Serialize, serde_derive::Deserialize, PartialEq, Eq, Hash, Copy,
// )]
// pub enum WriteLogsResult {
//     Success,
//     PartialSuccess, //todo: add values in here
//     Failure,
// }

// #[deny(
//     clippy::indexing_slicing,
//     clippy::unwrap_used,
//     clippy::integer_arithmetic,
//     clippy::float_arithmetic
// )]
// pub async fn async_write_fetch_error_logs_into_mongo_wrapper(
//     error_posts: Vec<PostErrorVariant>,
// ) -> WriteLogsResult {
//     if error_posts.is_empty() {
//         print_colorful_message(
//             None,
//             PrintType::WarningLow,
//             file!().to_string(),
//             line!().to_string(),
//             "error_posts.len() == 0".to_string(),
//         );
//         return WriteLogsResult::Success;
//     }
//     let time = Instant::now();
//     //todo: move this to config
//     let mut vec_of_error_provider_kinds: Vec<ProviderKind> = Vec::with_capacity(error_posts.len());
//     let mut hashmap_of_provider_vec_of_strings: HashMap<ProviderKind, Vec<String>> = HashMap::new();
//     //restructure into hashmap for better usage
//     //(link, are_there_items, provider_kind)
//     for post_error_variant in &error_posts {
//         match post_error_variant {
//             PostErrorVariant::NoItems {
//                 link: _,           //todo
//                 no_items_error: _, //todo
//                 provider_kind,
//             } => {
//                 if !vec_of_error_provider_kinds.contains(provider_kind) {
//                     let empty_vec_of_stringified_json: Vec<String> = Vec::new();
//                     hashmap_of_provider_vec_of_strings
//                         .insert(*provider_kind, empty_vec_of_stringified_json);
//                     vec_of_error_provider_kinds.push(*provider_kind);
//                 }
//             }
//             PostErrorVariant::RssFetchAndParseProviderDataError {
//                 link: _, //todo
//                 provider_kind,
//                 error: _, //it must be different type but dont know how to clone error to different thread
//             } => {
//                 if !vec_of_error_provider_kinds.contains(provider_kind) {
//                     let empty_vec_of_stringified_json: Vec<String> = Vec::new();
//                     hashmap_of_provider_vec_of_strings
//                         .insert(*provider_kind, empty_vec_of_stringified_json);
//                     vec_of_error_provider_kinds.push(*provider_kind);
//                 }
//             } //rewrite this error coz it must not be string. dont know to to clone error between threads
//         }
//     }
//     let hashmap_len = hashmap_of_provider_vec_of_strings.len();
//     if CONFIG.is_mongo_cleaning_warning_logs_db_enabled {
//         /////////////////////////////////////////////////////
//         //this error exists only for cloud mongo
//         //file: libs/mongo_integration/src/mongo_drop_db.rs:25
//         // Drop failed Error {
//         //     kind: CommandError(
//         //         CommandError {
//         //             code: 8000,
//         //             code_name: "AtlasError",
//         //             message: "user is not allowed to do action [dropDatabase] on [logs.]",
//         //             labels: [],
//         //         },
//         //     ),
//         //     labels: [],
//         // }
//         /////////////////////////////////////////////////////
//         //old tokio runtime
//         let dropping_db_result =
//             mongo_drop_db(&mongo_get_db_url(), &CONFIG.mongo_providers_logs_db_name).await;
//         match dropping_db_result {
//             Ok(_) => (),
//             Err(e) => {
//                 print_colorful_message(
//                     None,
//                     PrintType::Error,
//                     file!().to_string(),
//                     line!().to_string(),
//                     format!(
//                         "mongo_drop_db url: {}, db name: {}, error: {e:#?} \n maybe disable mongo dropping db parameter in config?",
//                         &mongo_get_db_url(), &CONFIG.mongo_providers_logs_db_name
//                     ),
//                 );
//                 return WriteLogsResult::Failure;
//             }
//         }
//     }
//     let mut vec_of_failed_collections_drops: Vec<ProviderKind> =
//         Vec::with_capacity(vec_of_error_provider_kinds.len());
//     if CONFIG.is_mongo_cleaning_warning_logs_db_collections_enabled {
//         let mut vec_join = Vec::new();
//         for pk in vec_of_error_provider_kinds {
//             vec_join.push(drop_mongo_provider_logs_collection_if_need(
//                 pk,
//                 mongo_get_db_url(),
//             ))
//         }
//         let result_vec = join_all(vec_join).await;
//         vec_of_failed_collections_drops = result_vec
//             .into_iter()
//             .filter(|(_, result)| result.is_err())
//             .map(|(pk, _)| -> ProviderKind { pk }) //todo: handle error
//             .collect();
//     }
//     ////(link, are_there_items, provider_kind)
//     for post_error_variant in error_posts {
//         ///////
//         match post_error_variant {
//             PostErrorVariant::NoItems {
//                 link,
//                 no_items_error,
//                 provider_kind,
//             } => {
//                 if !vec_of_failed_collections_drops.contains(&provider_kind) {
//                     let json = NoItemsError::into_json_with_link_and_provider_kind(
//                         &link.clone(), //todo understand lifetimes to remove it
//                         &no_items_error,
//                         &provider_kind,
//                     );
//                     let result_stringified_json = serde_json::to_string_pretty(&json);
//                     match result_stringified_json {
//                         Ok(stringified_json) => {
//                             match hashmap_of_provider_vec_of_strings.get_mut(&provider_kind) {
//                                 Some(stringified_json_vec) => {
//                                     stringified_json_vec.push(stringified_json)
//                                 }
//                                 None => {
//                                     print_colorful_message(
//                                         None,
//                                         PrintType::WarningHigh,
//                                         file!().to_string(),
//                                         line!().to_string(),
//                                         "hashmap_of_provider_vec_of_strings.get_mut(&provider_kind) is None"
//                                             .to_string(),
//                                     );
//                                 }
//                             }
//                         }
//                         Err(e) => {
//                             print_colorful_message(
//                                 Some(&provider_kind),
//                                 PrintType::WarningLow,
//                                 file!().to_string(),
//                                 line!().to_string(),
//                                 format!("serde_json::to_string_pretty(&json) error: {e:#?}"),
//                             );
//                             //todo
//                         }
//                     }
//                 }
//             }
//             PostErrorVariant::RssFetchAndParseProviderDataError {
//                 link,
//                 provider_kind,
//                 error, //it must be different type but dont know how to clone error to different thread
//             } => {
//                 if !vec_of_failed_collections_drops.contains(&provider_kind) {
//                     let json = json!({
//                         "link": link,
//                         "stringified_error": error.to_string(),
//                         "part_of": format!("{provider_kind}"),
//                         "date": Local::now().to_string()
//                     });
//                     let result_stringified_json = serde_json::to_string_pretty(&json);
//                     match result_stringified_json {
//                         Ok(stringified_json) => {
//                             match hashmap_of_provider_vec_of_strings.get_mut(&provider_kind) {
//                                 Some(stringified_json_vec) => {
//                                     stringified_json_vec.push(stringified_json)
//                                 }
//                                 None => {
//                                     print_colorful_message(
//                                         None,
//                                         PrintType::WarningHigh,
//                                         file!().to_string(),
//                                         line!().to_string(),
//                                         "hashmap_of_provider_vec_of_strings.get_mut(&provider_kind) is None"
//                                             .to_string(),
//                                     );
//                                 }
//                             }
//                         }
//                         Err(e) => {
//                             print_colorful_message(
//                                 Some(&provider_kind),
//                                 PrintType::WarningLow,
//                                 file!().to_string(),
//                                 line!().to_string(),
//                                 format!("serde_json::to_string_pretty(&json) error: {e:#?}"),
//                             );
//                             //todo
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     let mut vec_of_futures = Vec::with_capacity(hashmap_of_provider_vec_of_strings.len());
//     for element in hashmap_of_provider_vec_of_strings {
//         let collection_handle = format!(
//             "{:#?}{}",
//             element.0.clone(),
//             &CONFIG.mongo_providers_logs_db_collection_handle_second_part
//         );
//         //if push mongo_insert_docs_in_empty_collection then cant do join_all()
//         vec_of_futures.push(mongo_insert_docs_in_empty_collection(
//             &CONFIG.mongo_providers_logs_db_name,
//             collection_handle, //fix naming later
//             element.1,
//         ));
//     }
//     //todo write some logic around provider_kind
//     let results_vec = join_all(vec_of_futures).await;
//     if CONFIG.is_time_measurement_prints_enabled {
//         print_colorful_message(
//             None,
//             PrintType::TimeMeasurement,
//             file!().to_string(),
//             line!().to_string(),
//             format!(
//                 "write fetch error logs into files done in {} seconds {} miliseconds",
//                 time.elapsed().as_secs(),
//                 time.elapsed().as_millis()
//             ),
//         );
//     };
//     let results_vec_len = results_vec.len();
//     if results_vec_len == hashmap_len {
//         let mut checker_if_all_true = true;
//         for result in &results_vec {
//             if result.is_err() {
//                 checker_if_all_true = false;
//                 break;
//             }
//         }
//         if checker_if_all_true {
//             return WriteLogsResult::Success;
//         }
//         let mut checker_if_all_false = true;
//         for result in results_vec {
//             if result.is_ok() {
//                 checker_if_all_false = false;
//                 break;
//             }
//         }
//         if checker_if_all_false {
//             return WriteLogsResult::Failure;
//         }
//         //todo write more info about this case
//         print_colorful_message(
//             None,
//             PrintType::WarningLow,
//             file!().to_string(),
//             line!().to_string(),
//             "partial_success coz results_vec not all true".to_string(),
//         );
//         WriteLogsResult::PartialSuccess
//     } else {
//         let mut checker_if_all_false = true;
//         for result in results_vec {
//             if result.is_ok() {
//                 checker_if_all_false = false;
//                 break;
//             }
//         }
//         if checker_if_all_false {
//             return WriteLogsResult::Failure;
//         }
//         //todo write more info about this case
//         print_colorful_message(
//                     None,
//                     PrintType::WarningLow,
//                     file!().to_string(),
//                     line!().to_string(),
//                     format!("partial_success coz results_vec_len != hashmap_len and results_vec not all true, results_vec.len() {results_vec_len}, hashmap_len {hashmap_len}"),
//                 );
//         WriteLogsResult::PartialSuccess
//     }
// }
