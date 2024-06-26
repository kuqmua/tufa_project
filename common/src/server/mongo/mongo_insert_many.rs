// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum MongoInsertManyErrorNamed {
//     InsertMany {
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
//         insert_many: std::collections::HashMap<std::string::String, MongoInsertManyErrorUnnamed<'a>>,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum MongoInsertManyErrorUnnamed<'a> {
//     InsertMany(MongoInsertManyHandleErrorNamed),
// }

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum MongoInsertManyHandleErrorNamed {
//     InsertMany {
//         #[eo_to_std_string_string]
//         insert_many: mongodb::error::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }

// pub async fn mongo_insert_many<'a>(
//     providers_json_local_data_hashmap: std::collections::HashMap<crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
//     db: mongodb::Database,
//     config: &'static (
// impl app_state::GetMongoProvidersLogsDbCollectionDocumentFieldNameHandle
// + std::marker::Send
// + std::marker::Sync
// )
// ) -> Result<(), Box<crate::server::mongo::mongo_insert_many::MongoInsertManyErrorNamed>> {
//     let error_vec_insert_many = futures::future::join_all(
//         providers_json_local_data_hashmap.iter().map(
//                 |(pk, data_vec)|
//                 async {
//                     (
//                         pk.to_string(),
//                         db
//                         .collection(&{
//                             use crate::repositories_types::server::traits::provider_kind_methods::ProviderKindMethods;
//                             pk.get_db_tag()
//                         })
//                         .insert_many(
//                             data_vec
//                             .iter()
//                             .map(|data|
//                                 mongodb::bson::doc! { "data": data }//maybe "data" was without quotes
//                             )
//                             .collect::<Vec<mongodb::bson::Document>>(),
//                             None
//                         ).await
//                     )
//                 }
//             )
//         ).await
//         .into_iter()
//         .filter_map(|(pk, result)| {
//         if let Err(error) = result {
//             return Some((
//                 pk.to_string(),
//                 crate::server::mongo::mongo_insert_many::MongoInsertManyErrorUnnamed::InsertMany(
//                     crate::server::mongo::mongo_insert_many::MongoInsertManyHandleErrorNamed::InsertMany {
//                         insert_many: error,
//                         code_occurence: error_occurence_lib::code_occurence!()
//                     }
//                 )
//             ));
//         }
//         None
//     })
//     .collect::<std::collections::HashMap<String, crate::server::mongo::mongo_insert_many::MongoInsertManyErrorUnnamed>>();
//     match error_vec_insert_many.is_empty() {
//         true => Ok(()),
//         false => Err(Box::new(
//             crate::server::mongo::mongo_insert_many::MongoInsertManyErrorNamed::InsertMany {
//                 insert_many: error_vec_insert_many,
//                 code_occurence: error_occurence_lib::code_occurence!()
//             },
//         ))
//     }
// }
