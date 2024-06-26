// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum PostgresCheckProvidersLinkPartsTablesEmptyErrorNamed {
//     SelectCountOrigin {
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
//         hashmap_provider_kind_sqlx_error: std::collections::HashMap<std::string::String, sqlx::Error>,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
//     NotEmptyOrigin {
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         hashmap_provider_kind_len: std::collections::HashMap<std::string::String, i64>,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     }
// }

// pub async fn postgres_check_providers_link_parts_tables_are_empty<'a>(
//     providers_json_local_data_hashmap: &std::collections::HashMap<crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
//     postgres_pool: &sqlx::Pool<sqlx::Postgres>,
// ) -> Result<(), Box<crate::server::postgres::postgres_check_providers_link_parts_tables_are_empty::PostgresCheckProvidersLinkPartsTablesEmptyErrorNamed>> {
//     let count_provider_links_tables_tasks_vec =
//         providers_json_local_data_hashmap.keys().map(|pk| async {
//             let query_string = format!(
//                 "SELECT count(*) AS exact_count FROM {};",
//                 {
//                     use crate::repositories_types::server::traits::provider_kind_methods::ProviderKindMethods;
//                     pk.get_postgres_table_name()
//                 }
//             );
//             (*pk, sqlx::query_as(&query_string).fetch_one(postgres_pool).await)
//         });
//     let count_provider_links_tables_error_vec: Vec<(crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind, Result<(i64,), sqlx::Error>)> =
//         futures::future::join_all(count_provider_links_tables_tasks_vec).await;
//     let mut count_provider_links_tables_error_hashmap: std::collections::HashMap<std::string::String, sqlx::Error> =
//         std::collections::HashMap::with_capacity(count_provider_links_tables_error_vec.len());
//     let mut provider_links_tables_not_empty_error_hashmap: std::collections::HashMap<std::string::String, i64> =
//         std::collections::HashMap::with_capacity(count_provider_links_tables_error_vec.len());
//     for (pk, result) in count_provider_links_tables_error_vec {
//         match result {
//             Err(error) => {
//                 count_provider_links_tables_error_hashmap.insert(pk.to_string(), e);
//             }
//             Ok((count,)) => {
//                 if count.is_positive() {
//                     provider_links_tables_not_empty_error_hashmap.insert(pk.to_string(), count);
//                 }
//             }
//         }
//     }
//     if !count_provider_links_tables_error_hashmap.is_empty() {
//         return Err(Box::new(
//             crate::server::postgres::postgres_check_providers_link_parts_tables_are_empty::PostgresCheckProvidersLinkPartsTablesEmptyErrorNamed::SelectCountOrigin {
//                 hashmap_provider_kind_sqlx_error: count_provider_links_tables_error_hashmap,
//                 code_occurence: error_occurence_lib::code_occurence!()
//             }
//         ));
//     }
//     if !provider_links_tables_not_empty_error_hashmap.is_empty() {
//         return Err(Box::new(
//             crate::server::postgres::postgres_check_providers_link_parts_tables_are_empty::PostgresCheckProvidersLinkPartsTablesEmptyErrorNamed::NotEmptyOrigin {
//                 hashmap_provider_kind_len: provider_links_tables_not_empty_error_hashmap,
//                 code_occurence: error_occurence_lib::code_occurence!()
//             }
//         ));
//     }
//     Ok(())
// }
