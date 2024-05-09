// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
// pub enum MongoCheckCollectionIsNotEmptyErrorNamed {
//     Mongo {
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
//         inner_errors:
//             std::collections::HashMap<std::string::String, MongoCheckCollectionIsNotEmptyErrorUnnamed<'a>>,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
// pub enum MongoCheckCollectionIsNotEmptyErrorUnnamed<'a> {
//     CountDocumentsOrigin(MongoCheckCollectionIsNotEmptyErrorCountDocumentsErrorNamed),
//     IsNotEmptyOrigin(MongoCheckCollectionIsNotEmptyErrorIsNotEmptyOriginErrorNamed),
// }

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
// pub enum MongoCheckCollectionIsNotEmptyErrorCountDocumentsErrorNamed {
//     CountDocuments {
//         #[eo_to_std_string_string]
//         error: mongodb::error::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
// pub enum MongoCheckCollectionIsNotEmptyErrorIsNotEmptyOriginErrorNamed {
//     IsNotEmptyOrigin {
//         #[eo_to_std_string_string_serialize_deserialize]
//         error: u64,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }

// pub async fn mongo_check_collections_is_not_empty<'a>(
//     providers_json_local_data_hashmap: std::collections::HashMap<crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
//     db: &mongodb::Database,
// ) -> Result<(), Box<crate::server::mongo::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorNamed>>{
//     let error_vec_count_documents =
//         futures::future::join_all(providers_json_local_data_hashmap.keys().map(|pk| async move {
//             (
//                 *pk,
//                 db.collection::<mongodb::bson::Document>(&format!("{pk}"))
//                     .count_documents(None, None)
//                     .await,
//             )
//         }))
//         .await
//         .into_iter()
//         .filter_map(|(pk, result)| match result {
//             Err(error) => Some((
//                 pk.to_string(),
//                 crate::server::mongo::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorUnnamed::CountDocumentsOrigin(
//                     crate::server::mongo::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorCountDocumentsErrorNamed::CountDocuments {
//                         error,
//                         code_occurence: error_occurence_lib::code_occurence!(),
//                     }
//                 ),
//             )),
//             Ok(documents_number) => {
//                 if documents_number.is_positive() {
//                     return Some((
//                         pk.to_string(),
//                         crate::server::mongo::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorUnnamed::IsNotEmptyOrigin(
//                             crate::server::mongo::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorIsNotEmptyOriginErrorNamed::IsNotEmptyOrigin {
//                                 error: documents_number,
//                                 code_occurence: error_occurence_lib::code_occurence!()
//                             }
//                         )
//                     ));
//                 }
//                 None
//             }
//         })
//         .collect::<std::collections::HashMap<String, crate::server::mongo::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorUnnamed>>();
//     if !error_vec_count_documents.is_empty() {
//         return Err(Box::new(
//             crate::server::mongo::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorNamed::Mongo {
//                 inner_errors: error_vec_count_documents,
//                 code_occurence: error_occurence_lib::code_occurence!()
//             }
//         ));
//     }
//     Ok(())
// }
