// #[derive(Debug)]
// pub struct RemoveDirError {
//     pub error: std::io::Error,
// }

// #[derive(Debug)]
// pub enum CleanLogsDirError {
//     PathIsNotDir { path: std::string::String },
//     CannotRemoveDir { error: RemoveDirError },
// }
// impl From<String> for CleanLogsDirError {
//     fn from(e: std::string::String) -> Self {
//         CleanLogsDirError::PathIsNotDir { path: e }
//     }
// }
// impl From<std::io::Error> for CleanLogsDirError {
//     fn from(e: std::io::Error) -> Self {
//         CleanLogsDirError::CannotRemoveDir {
//             error: RemoveDirError { error: e },
//         }
//     }
// }

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum FetchAndParseProviderDataErrorNamed {
//     // AsyncFetchLinks {
//     //     #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
//     //     fetch_links: std::collections::HashMap<std::string::String, crate::server::http_request::http_request_error::HttpRequestErrorNamed>, //link, error
//     //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     // },
//     NoItems {
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
//         no_items: std::collections::HashMap<std::string::String, FetchAndParseProviderDataErrorUnnamed<'a>>, //link, error
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum FetchAndParseProviderDataErrorUnnamed<'a> {
//     NoItems(crate::repositories_types::server::fetch::rss_metainfo_fetch_structures::NoItemsErrorNamed),
// }

// impl ProviderKind {
//     pub async fn fetch_and_parse_provider_data<'a>(
//         self,
//         links: Vec<String>,
//     ) -> Result<Vec<crate::repositories_types::server::fetch::info_structures::common_rss_structures::CommonRssPostStruct>, Box<FetchAndParseProviderDataErrorNamed>> {
//         let time = std::time::Instant::now();
//         let capacity = links.len();
//         let vec_to_return = futures::future::join_all(links.iter().map(|url| async move {
//             let result = crate::server::http_request::wrappers::text::async_http_request_text::async_http_request_text_wrapper::<
//                 std::string::String,
//                 reqwest::cookie::Jar,
//                 core::time::Duration,
//                 u32,
//                 u32,
//                 u32,
//                 std::time::Duration,
//                 std::net::IpAddr,
//                 std::time::Duration,
//                 std::string::String, //todo - dyn std::any::Any
//                 std::string::String,
//                 std::string::String,
//                 std::string::String,
//                 std::string::String,
//                 std::string::String,
//                 std::string::String,
//                 std::string::String,
//                 std::string::String,
//                 std::string::String,
//             >(
//                 url,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 crate::server::http_request::http_request_method::HttpRequestMethod::Get,
//                 &crate::global_variables::runtime::config::CONFIG.source_place_type,
//                 false,
//             )
//             .await;
//             (url, result)
//         }))
//         .await;
//         let mut half_success_vec = Vec::with_capacity(capacity);
//         let mut async_fetch_links_error_vec = Vec::new();
//         for (link, result) in vec_to_return {
//             match result {
//                 Err(error) => {
//                     async_fetch_links_error_vec.push((link.to_string(), e));
//                 }
//                 Ok(str) => {
//                     half_success_vec.push((link, str));
//                 }
//             }
//         }
//         if !async_fetch_links_error_vec.is_empty() {
//             //todo: maybe not all links must return Ok ?
//             return Err(
//                 FetchAndParseProviderDataErrorNamed::AsyncFetchLinks {
//                     fetch_links: async_fetch_links_error_vec,
//                     code_occurence: error_occurence_lib::code_occurence!()
//                 },
//             );
//         }
//         let mut success_vec = Vec::with_capacity(capacity);
//         let mut no_items_error_vec = Vec::new();
//         for (link, response_text) in half_success_vec {
//             match crate::repositories_types::server::fetch::rss_parse_string_into_struct::rss_parse_string_into_struct(response_text, link, self) {
//                 Err(error) => no_items_error_vec.push((link.to_string(), FetchAndParseProviderDataErrorUnnamed::NoItems(e))),
//                 Ok(post_struct) => {
//                     success_vec.push(post_struct); //todo maybe add link here?
//                 }
//             }
//         }
//         if !no_items_error_vec.is_empty() {
//             return Err(Box::new(FetchAndParseProviderDataErrorNamed::NoItems {
//                 no_items: no_items_error_vec,
//                 code_occurence: error_occurence_lib::code_occurence!()
//             }));
//         }
//         Ok(success_vec)
//     }
// }

// impl ProviderKind {
//     pub fn get_mongo_provider_link_parts_aggregation(&self) -> Option<mongodb::bson::Document> {
//         // if
//         //     *config.get_is_links_limit_enabled_providers()
//         //     &&
//         //     *self.is_mongo_link_parts_randomize_order_enabled(config)
//         // {
//         //     Some(mongodb::bson::doc! { "$sample" : {"size": *config.get_links_limit_providers() as i64}});
//         // } else if *config.get_is_links_limit_enabled_providers() {
//         //     Some(mongodb::bson::doc! { "$limit" :  *config.get_links_limit_providers() as i64});
//         // } else if *self.is_links_limit_enabled(config)
//         //     && *self.is_mongo_link_parts_randomize_order_enabled(config)
//         // {
//         //     Some(mongodb::bson::doc! { "$sample" : {"size": *self.links_limit(config) as i64}});
//         // } else if *self.is_links_limit_enabled(config) {
//         //     Some(mongodb::bson::doc! { "$limit" : *self.links_limit(config) as i64});
//         // } else if *self.is_mongo_link_parts_randomize_order_enabled(config) {
//         //     println!("todo: mongo sample(randomized aggregation) only works if size is valid number. No aggregation applied");
//         //     return None;
//         // }
//         None
//     }
// }

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum GetLinkPartsFromLocalJsonFileErrorNamed {
//     TokioFsFileOpen {
//         #[eo_to_std_string_string]
//         tokio_fs_file_open: std::io::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     TokioIoAsyncReadExtReadToEnd {
//         #[eo_to_std_string_string]
//         tokio_io_async_read_ext_read_to_end: std::io::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     SerdeJsonFromSlice {
//         #[eo_to_std_string_string]
//         serde_json_from_slice: serde_json::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }

// #[derive(
//     Default, Debug, Clone, PartialEq, Eq, serde_derive::Serialize, serde_derive::Deserialize,
// )]
// pub struct ProvidersInitJsonSchema {
//     pub data: Vec<String>,
// }

// impl ProviderKind {
//     pub async fn get_link_parts_from_local_json_file<'a>(
//         self,
//         config: &'static (
//             impl app_state::GetPathToProviderLinkPartsFolder
//             + app_state::GetLogFileExtension
// + std::marker::Send
// + std::marker::Sync
//         )
//     ) -> Result<Vec<String>, Box<GetLinkPartsFromLocalJsonFileErrorNamed>> {
//         match tokio::fs::File::open(&{
//             use crate::repositories_types::server::traits::provider_kind_methods::ProviderKindMethods;
//             self.get_init_local_data_file_path(config)
//         }).await {
//             Err(error) => Err(Box::new(
//                 GetLinkPartsFromLocalJsonFileErrorNamed::TokioFsFileOpen {
//                     tokio_fs_file_open: error,
//                     code_occurence: error_occurence_lib::code_occurence!(),
//                 }
//             )),
//             Ok(mut file) => {
//                 let mut content = Vec::new();
//                 if let Err(error) = tokio::io::AsyncReadExt::read_to_end(&mut file, &mut content).await
//                 {
//                     return Err(Box::new(
//                         GetLinkPartsFromLocalJsonFileErrorNamed::TokioIoAsyncReadExtReadToEnd {
//                             tokio_io_async_read_ext_read_to_end: error,
//                             code_occurence: error_occurence_lib::code_occurence!()
//                         }
//                     ));
//                 }
//                 match serde_json::from_slice::<ProvidersInitJsonSchema>(&content) {
//                     Err(error) => Err(Box::new(
//                         GetLinkPartsFromLocalJsonFileErrorNamed::SerdeJsonFromSlice {
//                             serde_json_from_slice: error,
//                             code_occurence: error_occurence_lib::code_occurence!()
//                         }
//                     )),
//                     Ok(file_content_as_struct) => {
//                         let unique_vec: Vec<String> =
//                             {
//                                 use itertools::Itertools;
//                                 file_content_as_struct.data.into_iter().unique()
//                             }.collect();
//                         let return_vec: Vec<String>;
//                         //todo - add correct impl for is_links_limit_enabled - like is_links_limit_enabled_providers && is_links_limit_enabled_arxiv
//                         if
//                             // *config.get_is_links_limit_enabled_providers()
//                             false
//                             &&
//                             // *self.is_links_limit_enabled(config)
//                             false
//                         {
//                             // let limit = self.links_limit(config);
//                             let limit = &1;
//                             if unique_vec.len() > *limit {
//                                 return_vec = unique_vec
//                                     .into_iter()
//                                     .enumerate()
//                                     .filter_map(|(index, element)| match index > *limit {
//                                         false => None,
//                                         true => Some(element),
//                                     })
//                                     .collect::<Vec<String>>();
//                             } else {
//                                 return_vec = unique_vec;
//                             }
//                         } else {
//                             return_vec = unique_vec;
//                         }
//                         Ok(return_vec)
//                     }
//                 }
//             }
//         }
//     }
// }

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum MongoGetProviderLinkPartsErrorNamed {
//     ClientOptionsParse {
//         #[eo_to_std_string_string]
//         mongo: mongodb::error::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     ClientWithOptions {
//         #[eo_to_std_string_string]
//         mongo: mongodb::error::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     // MongoGetDocumentsAsStringVector {
//     //    #[eo_error_occurence]
//     //     source: Box<MongoGetDocumentsAsStringVectorErrorEnum>,
//     //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     // },
// }
