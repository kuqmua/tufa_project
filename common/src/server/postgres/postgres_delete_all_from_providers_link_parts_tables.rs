// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum PostgresDeleteAllFromProvidersTablesErrorNamed {
//     DeleteTables {
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
//         error_hashmap: std::collections::HashMap<std::string::String, sqlx::Error>,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }

// pub async fn postgres_delete_all_from_providers_link_parts_tables<'a>(
//     providers_json_local_data_hashmap: &std::collections::HashMap<crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
//     postgres_pool: &sqlx::Pool<sqlx::Postgres>,
// ) -> Result<(), Box<crate::server::postgres::postgres_delete_all_from_providers_link_parts_tables::PostgresDeleteAllFromProvidersTablesErrorNamed>> {
//     let delete_from_tables_error_hashmap =
//         futures::future::join_all(providers_json_local_data_hashmap.keys().map(|pk| async {
//             let query_string = format!("DELETE FROM {} ;", {
//                 use crate::repositories_types::server::traits::provider_kind_methods::ProviderKindMethods;
//                 pk.get_postgres_table_name()
//             });
//             (*pk, sqlx::query(&query_string).execute(postgres_pool).await)
//         }))
//         .await
//         .into_iter()
//         .filter_map(|(pk, result)| {
//             if let Err(error) = result {
//                 return Some((pk.to_string(), e));
//             }
//             None
//         })
//         .collect::<std::collections::HashMap<std::string::String, sqlx::Error>>();
//     if !delete_from_tables_error_hashmap.is_empty() {
//         return Err(Box::new(
//             crate::server::postgres::postgres_delete_all_from_providers_link_parts_tables::PostgresDeleteAllFromProvidersTablesErrorNamed::DeleteTables {
//                 error_hashmap: delete_from_tables_error_hashmap,
//                 code_occurence: error_occurence_lib::code_occurence!()
//             }
//         ));
//     }
//     Ok(())
// }
