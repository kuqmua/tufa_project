// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum PostgresInsertLinkPartsIntoProvidersTablesOriginErrorNamed {
//     Postgres {
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
//         inner_errors: std::collections::HashMap<
//             std::string::String,
//             PostgresInsertLinkPartsIntoProvidersTablesOriginErrorEnumUnnamed<'a>,
//         >,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum PostgresInsertLinkPartsIntoProvidersTablesOriginErrorEnumUnnamed<'a> {
//     PostgresInsertLinkPartsIntoProvidersTablesOriginHandle(PostgresInsertLinkPartsIntoProvidersTablesOriginHandleErrorNamed),
// }

// #[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
// pub enum PostgresInsertLinkPartsIntoProvidersTablesOriginHandleErrorNamed {
//     Postgres {
//         #[eo_to_std_string_string]
//         error: sqlx::Error,
//         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
//     },
// }

// pub async fn postgres_insert_link_parts_into_providers_tables<'a>(
//     providers_json_local_data_hashmap: &std::collections::HashMap<crate::repositories_types::server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
//     postgres_pool: &sqlx::Pool<sqlx::Postgres>,
// ) -> Result<(), Box<crate::server::postgres::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesOriginErrorNamed>>{
//     let insertion_error_hashmap = futures::future::join_all(providers_json_local_data_hashmap.iter().map(
//         |(pk, string_vec)| async {
//             let mut values_string = std::string::String::from("");
//             for link_part in string_vec.clone() {
//                 values_string.push_str(&format!("('{link_part}'),"));
//             }
//             if !values_string.is_empty() {
//                 values_string.pop();
//             }
//             let query_string = format!(
//                 "INSERT INTO {} (link_part) VALUES {values_string};",
//                 {
//                     use crate::repositories_types::server::traits::provider_kind_methods::ProviderKindMethods;
//                     pk.get_postgres_table_name()
//                 }
//             );
//             (*pk, sqlx::query(&query_string).execute(postgres_pool).await)
//         },
//     ))
//     .await
//     .into_iter()
//     .filter_map(|(pk, result)| {
//         if let Err(error) = result {
//             return Some((
//                 pk.to_string(),
//                 crate::server::postgres::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesOriginErrorEnumUnnamed::PostgresInsertLinkPartsIntoProvidersTablesOriginHandle(
//                     crate::server::postgres::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesOriginHandleErrorNamed::Postgres {
//                         error,
//                         code_occurence: error_occurence_lib::code_occurence!()
//                     }
//                 )
//             ));
//         }
//         None
//     })
//     .collect::<std::collections::HashMap<
//         std::string::String, crate::server::postgres::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesOriginErrorEnumUnnamed
//     >>();
//     if !insertion_error_hashmap.is_empty() {
//         return Err(Box::new(
//             crate::server::postgres::postgres_insert_link_parts_into_providers_tables::PostgresInsertLinkPartsIntoProvidersTablesOriginErrorNamed::Postgres {
//                 inner_errors: insertion_error_hashmap,
//                 code_occurence: error_occurence_lib::code_occurence!()
//             }
//         ));
//     }
//     Ok(())
// }
