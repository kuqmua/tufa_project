// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum GetProvidersLinkPartsErrorNamed {
//     GetLocalProvidersLinkParts {
//         #[eo_error_occurence]
//         get_local_providers_link_parts: crate::repositories_types::server::providers::providers_info::get_local_providers_link_parts::GetLocalProvidersLinkPartsErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     MongoGetProvidersLinkParts {
//          #[eo_error_occurence]
//         mongo_get_providers_link_parts: crate::server::mongo::mongo_get_providers_link_parts::MongoGetProvidersLinkPartsErrorNamed,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
//     // PostgreSql {
//     //     // source: PostgresGetProviderLinksError,
//     //     // code_occurence: crate::common::code_occurence::CodeOccurence,
//     // },
// }

// pub async fn get_providers_link_parts<'a>(
//     config: &'static (
//         impl config_lib::config_fields::GetProvidersLinkPartsSource

//         + config_lib::config_fields::GetPathToProviderLinkPartsFolder
//         + config_lib::config_fields::GetLogFileExtension

//         + config_lib::config_fields::GetMongoUrl
//         + config_lib::config_fields::GetMongoProvidersLinkPartsDbName
//         + config_lib::config_fields::GetMongoProvidersLogsDbCollectionDocumentFieldNameHandle
//         + config_lib::config_fields::GetMongoProvidersLogsDbCollectionHandleSecondPart
// + std::marker::Send
// + std::marker::Sync
//     )
// ) -> Result<std::collections::HashMap<crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>, Box<GetProvidersLinkPartsErrorNamed>> {
//     match
// config.get_providers_link_parts_source() //postgres mongo local
// {
//         crate::server::resource::Resource::Local => match crate::repositories_types::server::providers::providers_info::get_local_providers_link_parts::get_local_providers_link_parts(config).await {
//             Err(error_hashmap) => Err(Box::new(
//                 GetProvidersLinkPartsErrorNamed::GetLocalProvidersLinkParts {
//                     get_local_providers_link_parts: *error_hashmap,
//                     code_occurence: crate::code_occurence_common!(),
//                 }
//             )),
//             Ok(success_hashmap) => Ok(success_hashmap),
//         },
//         crate::server::resource::Resource::Mongodb => match crate::server::mongo::mongo_get_providers_link_parts::mongo_get_providers_link_parts(config).await {
//             Err(e) => Err(Box::new(

//                 GetProvidersLinkPartsErrorNamed::MongoGetProvidersLinkParts {
//                     mongo_get_providers_link_parts: e,
//                     code_occurence: crate::code_occurence_common!(),
//                 },
//             )),
//             Ok(success_hashmap) => Ok(success_hashmap),
//         },
//         crate::server::resource::Resource::PostgreSql => todo!(),
//     }
// }
