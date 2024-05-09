// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
// pub enum MongoGetProvidersLinkPartsErrorNamed {
//     MongoDB {
//         #[eo_to_std_string_string]
//         mongodb: mongodb::error::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     NoSuchCollections {
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         no_such_collections: std::collections::HashMap<std::string::String, std::string::String>,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     GetDocuments {
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
//         get_documents: std::collections::HashMap<std::string::String, MongoGetDocumentsAsStringVectorErrorUnnamed<'a>>,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
// pub enum MongoGetDocumentsAsStringVectorErrorUnnamed<'a> {
//     MongoGetDocumentsAsStringVector(crate::server::mongo::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorErrorNamed),
// }

// pub async fn mongo_get_providers_link_parts<'a>(
//     config: &'static (
//         impl app_state::GetMongoUrl
//         + app_state::GetMongoProvidersLinkPartsDbName
//         + app_state::GetMongoProvidersLogsDbCollectionDocumentFieldNameHandle
//         + app_state::GetMongoProvidersLogsDbCollectionHandleSecondPart
//         + app_state::GetMongoClient
//         + std::marker::Send
//         + std::marker::Sync
//     )
// ) -> Result<
//         std::collections::HashMap<
//                 crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind,
//                 Vec<String>
//         >,
//         crate::server::mongo::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed
//     > {
//         let db = config.get_mongo_client().database("providers_link_parts");
//         match db.list_collection_names(None).await {
//             Err(error) => Err(
//                 crate::server::mongo::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed::MongoDB {
//                     mongodb: error,
//                     code_occurence: error_occurence_lib::code_occurence!(),
//                 }
//             ),
//             Ok(vec_collection_names) => {
//                 let no_collection_error_hashmap = {
//                     use crate::repositories_types::server::traits::provider_kind_methods::ProviderKindMethods;
//                     crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind::get_enabled_providers_vec()
//                     .into_iter()
//                     .filter_map(|pk| {
//                         let collection_name = pk.get_mongo_log_collection_name(config);
//                         if !vec_collection_names.contains(&collection_name) {
//                             return Some((pk.to_string(), collection_name));
//                         }
//                         None
//                     })
//                     .collect::<std::collections::HashMap<String, std::string::String>>()
//                 };
//                 if !no_collection_error_hashmap.is_empty() {
//                     return Err(
//                         crate::server::mongo::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed::NoSuchCollections {
//                             no_such_collections: no_collection_error_hashmap,
//                             code_occurence: error_occurence_lib::code_occurence!(),
//                         }
//                     );
//                 }
//                 let result_get_documents_hashmap =
//                     futures::future::join_all({
//                         use crate::repositories_types::server::traits::provider_kind_methods::ProviderKindMethods;
//                         crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind::get_enabled_providers_vec().iter().map(|pk| async {
//                             (
//                                 *pk,
//                                 crate::server::mongo::mongo_get_documents_as_string_vector::mongo_get_documents_as_string_vector(
//                                     db.collection::<mongodb::bson::Document>(&pk.get_mongo_log_collection_name(config)),
//                                     "data",
//                                     crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind::get_mongo_provider_link_parts_aggregation(pk),
//                                 )
//                                 .await,
//                             )
//                         })
//                     })
//                     .await;
//                 let mut success_hashmap: std::collections::HashMap<crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>> =
//                     std::collections::HashMap::with_capacity(result_get_documents_hashmap.len());
//                 let mut error_hashmap: std::collections::HashMap<
//                     std::string::String,
//                     crate::server::mongo::mongo_get_providers_link_parts::MongoGetDocumentsAsStringVectorErrorUnnamed,
//                 > = std::collections::HashMap::new();
//                 for (pk, result) in result_get_documents_hashmap.into_iter() {
//                     match result {
//                         Err(error) => {
//                             error_hashmap.insert(
//                                 pk.to_string(),
//                                 crate::server::mongo::mongo_get_providers_link_parts::MongoGetDocumentsAsStringVectorErrorUnnamed::MongoGetDocumentsAsStringVector(*e)
//                             );
//                         }
//                         Ok(vec) => {
//                             success_hashmap.insert(pk, vec);
//                         }
//                     }
//                 }
//                 if !error_hashmap.is_empty() {
//                     return Err(
//                         crate::server::mongo::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed::GetDocuments {
//                             get_documents: error_hashmap,
//                             code_occurence: error_occurence_lib::code_occurence!(),
//                         }
//                     );
//                 }
//                 Ok(success_hashmap)
//             }
//         }
// }
