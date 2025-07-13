// // use crate::write_error_posts_wrapper::write_error_posts_wrapper;

// // use crate::fetch::rss_filter_fetched_and_parsed_posts::PostErrorVariant;
// // use reqwest::StatusCode;
// // #[derive(Debug)]
// // pub enum PostErrorVariant {
// //     //todo: think about this naming
// //     NoItems {
// //         link: std::string::String,
// //         no_items_error: NoItemsError,
// //         provider_kind: crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind,
// //     },
// //     RssFetchAndParseProviderDataError {
// //         link: std::string::String,
// //         provider_kind: crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind,
// //         error: RssFetchLinkError,
// //     }, //rewrite this error coz it must not be string. dont know to to clone error between threads
// // }

// // #[derive(Debug)]
// // pub enum RssPartError {
// //     ReqwestError(reqwest::Error),
// //     StatusCode(StatusCode),
// // }
// //TODO: WRITE CONVERSION FUNCTION INTO COMMON ERROR ENUM AND MOVE IT INTO write_error_posts_wrapper

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum GetProviderPostsErrorNamed {
//     // GetLocalProvidersLinkParts {
//     //     #[eo_error_occurence]
//     //     get_providers_link_parts: crate::repositories_types::server::providers::providers_info::get_providers_link_parts::GetProvidersLinkPartsErrorNamed,
//     //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     // },
//     // CheckProvidersLinkPartsEmpty {
//     //     #[eo_error_occurence]
//     //     check_providers_link_parts_empty: crate::repositories_types::server::providers::check_providers_link_parts_on_empty::CheckProvidersLinkPartsEmptyErrorNamed,
//     //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     // },
//     GetNewProvidersPosts {
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
//         hashmap_provider_kind_rss_part: std::collections::HashMap<std::string::String, GetProviderPostsErrorUnnamed<'a>>,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum GetProviderPostsErrorUnnamed<'a> {
//     GetNewProvidersPosts(crate::repositories_types::server::providers::provider_kind::functions::rss_part::RssPartErrorNamed),
// }

// pub async fn get_providers_posts<'a>(
//     config: &'static (
//         impl app_state::GetGithubToken
//         + app_state::GetProvidersLinkPartsSource

//         + app_state::GetPathToProviderLinkPartsFolder
//         + app_state::GetLogFileExtension
//         + app_state::GetMongoUrl
//         + app_state::GetMongoProvidersLinkPartsDbName
//         + app_state::GetMongoProvidersLogsDbCollectionDocumentFieldNameHandle
//         + app_state::GetMongoProvidersLogsDbCollectionHandleSecondPart
// + std::marker::Send
// + std::marker::Sync
//     )
// ) -> Result<(), Box<GetProviderPostsErrorNamed>> {
//     todo!()
//     // match crate::repositories_types::server::providers::providers_info::get_providers_link_parts::get_providers_link_parts(config).await {
//     //     Err(error) => Err(Box::new(
//     //         GetProviderPostsErrorNamed::GetLocalProvidersLinkParts {
//     //             get_providers_link_parts: *e,
//     //             code_occurence: error_occurence_lib::code_occurence!()
//     //         },
//     //     )),
//     //     Ok(providers_link_parts) => {
//     //         match crate::repositories_types::server::providers::check_providers_link_parts_on_empty::check_providers_link_parts_on_empty(providers_link_parts) {
//     //             Err(error) => {
//     //                 return Err(Box::new(
//     //                     GetProviderPostsErrorNamed::CheckProvidersLinkPartsEmpty {
//     //                         check_providers_link_parts_empty: *e,
//     //                         code_occurence: error_occurence_lib::code_occurence!()
//     //                     },
//     //                 ));
//     //             }
//     //             Ok(non_empty_providers_link_parts) => {
//     //                 let hm = crate::repositories_types::server::check_new_providers_posts::check_new_providers_posts(non_empty_providers_link_parts, config).await;
//     //                 let mut error_hashmap = std::collections::HashMap::with_capacity(hm.len());
//     //                 let mut success_hashmap = std::collections::HashMap::with_capacity(hm.len());
//     //                 for (key, value) in hm {
//     //                     match value {
//     //                         Err(error) => {
//     //                             error_hashmap.insert(key.to_string(), GetProviderPostsErrorUnnamed::GetNewProvidersPosts(e));
//     //                         }
//     //                         Ok(vec) => {
//     //                             success_hashmap.insert(key, vec);
//     //                         }
//     //                     }
//     //                 }
//     //                 if !error_hashmap.is_empty() {
//     //                     return Err(Box::new(GetProviderPostsErrorNamed::GetNewProvidersPosts {
//     //                         hashmap_provider_kind_rss_part: error_hashmap,
//     //                         code_occurence: error_occurence_lib::code_occurence!()
//     //                     }));
//     //                 }
//     //                 Ok(())
//     //             }
//     //         }
//     //         // //todo: conversion function before write_error_posts_wrapper
//     //         // //commented before conversion function implementation
//     //         // // if !vec.is_empty() {
//     //         // //     for (pk, result_vec) in vec {
//     //         // //         match result_vec {
//     //         // //             Ok((vec_common_rss_post_structs, vec_post_error_variants)) => {
//     //         // //                 let wrong_cases_thread = thread::spawn(move || {
//     //         // //                     block_on(write_error_posts_wrapper(vec_post_error_variants));
//     //         // //                 });
//     //         // //                 match wrong_cases_thread.join() {
//     //         // //                     Ok(_) => {}
//     //         // //                     Err(error) => {}
//     //         // //                 }
//     //         // //             }
//     //         // //             Err(error) => {}
//     //         // //         }
//     //         // //     }
//     //         // // }
//     //     }
//     // }
// }
