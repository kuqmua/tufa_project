// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum GetLocalProvidersLinkPartsErrorNamed {
//     GetLinkPartsFromLocalJsonFile {
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
//         get_link_parts_from_local_json_file: std::collections::HashMap<std::string::String, GetLocalProvidersLinkPartsErrorUnnamed<'a>>,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum GetLocalProvidersLinkPartsErrorUnnamed<'a> {
//     GetLinkPartsFromLocalJsonFile(crate::repositories_types::server::providers::provider_kind::provider_kind_enum::GetLinkPartsFromLocalJsonFileErrorNamed),
// }

// #[derive(Clone, Debug, valuable::Valuable)]
// pub struct TracingVec {
//     pub vec: Vec<String>,
// }

// pub async fn get_local_providers_link_parts<'a>(
//     config: &'static (
//         impl config_lib::GetPathToProviderLinkPartsFolder
//         + config_lib::GetLogFileExtension
// + std::marker::Send
// + std::marker::Sync
//     )
// ) -> Result<std::collections::HashMap<crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>, Box<GetLocalProvidersLinkPartsErrorNamed>> {
//     let result_vec = futures::future::join_all(
//             {
//                 use crate::repositories_types::server::traits::provider_kind_methods::ProviderKindMethods;
//                 crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind::get_enabled_providers_vec() //maybe its not exactly correct
//             }
//             .into_iter()
//             .map(|pk| async move {
//                 (
//                     pk,
//                     crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind::get_link_parts_from_local_json_file(
//                         pk,
//                         config
//                     ).await,
//                 )
//             }),
//     )
//     .await;
//     let mut errors_hashmap: std::collections::HashMap<std::string::String, GetLocalProvidersLinkPartsErrorUnnamed<'a>> =
//         std::collections::HashMap::new();
//     let mut success_hashmap: std::collections::HashMap<crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>> =
//         std::collections::HashMap::with_capacity(result_vec.len());
//     for (pk, result) in result_vec {
//         match result {
//             Err(e) => {
//                 errors_hashmap.insert(pk.to_string(), GetLocalProvidersLinkPartsErrorUnnamed::GetLinkPartsFromLocalJsonFile(*e));
//             }
//             Ok(vec) => {
//                 success_hashmap.insert(pk, vec);
//             }
//         }
//     }
//     if !errors_hashmap.is_empty() {
//         return Err(Box::new(
//             GetLocalProvidersLinkPartsErrorNamed::GetLinkPartsFromLocalJsonFile {
//                 get_link_parts_from_local_json_file: errors_hashmap,
//                 code_occurence: error_occurence_lib::code_occurence!(),
//             }
//         ));
//     }
//     Ok(success_hashmap)
// }
