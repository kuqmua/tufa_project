// pub trait ProviderKindMethods {
//     fn get_item_handle(&self) -> Option<&'static str>;
//     fn get_mongo_log_collection_name(&self, config: &'static (
// impl crate::common::config::config_fields::GetMongoProvidersLogsDbCollectionHandleSecondPart
// + std::marker::Send
// + std::marker::Sync
// )
// ) -> std::string::String;
//     fn get_init_local_data_file_path(
//         &self,
//         config: &'static (
//             impl crate::common::config::config_fields::GetPathToProviderLinkPartsFolder
//             + crate::common::config::config_fields::GetLogFileExtension
// + std::marker::Send
// + std::marker::Sync
//         )
//     ) -> std::string::String;
//     fn generate_provider_links(
//         &self,
//         names_vector: Vec<String>,
//         config: &'static (
//  impl crate::common::config::config_fields::GetGithubToken
// + std::marker::Send
// + std::marker::Sync
// )
//     ) -> Vec<String>;
//     fn generate_hashmap_with_empty_string_vecs_for_enabled_providers() -> std::collections::HashMap<Self, Vec<String>>
//     where
//         Self: Sized;
//     fn get_enabled_providers_vec() -> Vec<Self>
//     where
//         Self: Sized;
//     fn get_enabled_string_name_vec() -> Vec<String>;
//     fn get_mongo_initialization_provider_kind_vec() -> Vec<Self>
//     where
//         Self: Sized;
//     fn get_mongo_initialization_string_name_vec() -> Vec<String>;
//     fn into_string_name_and_kind_hashmap() -> std::collections::HashMap<String, Self>
//     where
//         Self: Sized;
//     fn get_db_tag(&self) -> std::string::String
//     where
//         Self: Sized;
//     fn get_postgres_table_name(&self) -> std::string::String
//     where
//         Self: Sized;
// }
