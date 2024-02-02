// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum InitMongoErrorNamed {
//     ClientOptionsParse {
//         #[eo_error_occurence]
//         client_options_parse: crate::server::mongo::mongo_client_options_parse::MongoClientOptionsParseOriginErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     ClientWithOptions {
//         #[eo_error_occurence]
//         client_with_options: crate::server::mongo::mongo_client_with_options::MongoClientWithOptionsOriginErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     CollectionIsNotEmpty {
//         #[eo_error_occurence]
//         collection_is_not_empty: crate::server::mongo::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     InsertMany {
//         #[eo_error_occurence]
//         insert_many: crate::server::mongo::mongo_insert_many::MongoInsertManyErrorNamed,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     }
// }

// pub async fn init_mongo<'a>(
//     providers_json_local_data_hashmap: std::collections::HashMap<crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
//     config: &'static (
//         impl config_lib::config_fields::GetMongoProvidersLinkPartsDbName
//         + config_lib::config_fields::GetMongoUrl
//         + config_lib::config_fields::GetMongoProvidersLogsDbCollectionDocumentFieldNameHandle
// + std::marker::Send
// + std::marker::Sync
//     ),
// ) -> Result<(), Box<crate::repositories_types::server::init_dbs_logic::init_mongo::InitMongoErrorNamed>>{
//     match crate::server::mongo::mongo_client_options_parse::mongo_client_options_parse(config).await {
//         Err(e) => Err(Box::new(
//             crate::repositories_types::server::init_dbs_logic::init_mongo::InitMongoErrorNamed::ClientOptionsParse {
//                 client_options_parse: *e,
//                 code_occurence: crate::code_occurence!()
//             }
//         )),
//         Ok(client_options) => match crate::server::mongo::mongo_client_with_options::mongo_client_with_options(client_options) {
//             Err(e) => Err(Box::new(
//                 crate::repositories_types::server::init_dbs_logic::init_mongo::InitMongoErrorNamed::ClientWithOptions {
//                     client_with_options: *e,
//                     code_occurence: crate::code_occurence!()
//                 }
//             )),
//             Ok(client) => {
//                 let db = client.database("providers_link_parts");
//                 if let Err(e) = crate::server::mongo::mongo_check_collection_is_not_empty::mongo_check_collections_is_not_empty(
//                     providers_json_local_data_hashmap.clone(),
//                     &db,
//                 )
//                 .await
//                 {
//                     return Err(Box::new(
//                         crate::repositories_types::server::init_dbs_logic::init_mongo::InitMongoErrorNamed::CollectionIsNotEmpty {
//                             collection_is_not_empty: *e,
//                             code_occurence: crate::code_occurence!()
//                         }
//                     ));
//                 }
//                 if let Err(e) =
//                     crate::server::mongo::mongo_insert_many::mongo_insert_many(providers_json_local_data_hashmap, db, config).await
//                 {
//                     return Err(Box::new(
//                         crate::repositories_types::server::init_dbs_logic::init_mongo::InitMongoErrorNamed::InsertMany {
//                             insert_many: *e,
//                             code_occurence: crate::code_occurence!()
//                         }
//                     ));
//                 }
//                 Ok(())
//             }
//         },
//     }
// }
