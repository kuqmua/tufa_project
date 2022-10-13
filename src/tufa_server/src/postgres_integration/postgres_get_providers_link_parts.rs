// use std::collections::HashMap;
// use diesel::pg::PgConnection;
// use diesel::prelude::*;
// use crate::postgres_integration::postgres_get_db_url::postgres_get_db_url;
// use crate::postgres_integration::models::queryable::queryable_link_part::QueryableLinkPart;
// use crate::postgres_integration::schemas::providers_link_parts_schema::providers_link_parts::dsl::*;
// use crate::lazy_static::config::CONFIG;
// use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
// use crate::traits::enum_extension::EnumExtension;

// #[derive(Debug)]
// pub enum PostgresGetProviderLinksError {
//     LoadingProvidersLinkParts(diesel::result::Error),
//     EstablishConnection(ConnectionError),
//     IncorrectProviderNameInsideDb(String),
// }

// #[deny(
//     clippy::indexing_slicing,
//     clippy::unwrap_used,
//     clippy::integer_arithmetic,
//     clippy::float_arithmetic
// )]
// pub async fn postgres_get_providers_link_parts(
// ) -> Result<HashMap<ProviderKind, Vec<String>>, PostgresGetProviderLinksError> {
//     match PgConnection::establish(&postgres_get_db_url()) {
//         Err(e) => Err(PostgresGetProviderLinksError::EstablishConnection(e)),
//         Ok(pg_connection) => {
//             let result = providers_link_parts
//                 // .filter()//todo for all providers use limits from config
//                 .limit(CONFIG.links_limit_providers)
//                 .load::<QueryableLinkPart>(&pg_connection);
//             match result {
//                 Err(e) => Err(PostgresGetProviderLinksError::LoadingProvidersLinkParts(e)),
//                 Ok(vec) => {
//                     let mut providers_vec_from_db = Vec::new();
//                     for i in &vec {
//                         let mut is_correct_field = false;
//                         for pk in ProviderKind::into_array() {
//                             if i.provider_kind == format!("{pk}") {
//                                 is_correct_field = true;
//                                 if !(providers_vec_from_db.contains(pk)) {
//                                     providers_vec_from_db.push(*pk);
//                                 }
//                                 break;
//                             }
//                         }
//                         if !is_correct_field {
//                             return Err(
//                                 PostgresGetProviderLinksError::IncorrectProviderNameInsideDb(
//                                     i.provider_kind.clone(),
//                                 ),
//                             );
//                         }
//                     }
//                     let mut hashmap_to_return: HashMap<ProviderKind, Vec<String>> = HashMap::new(); //todo with capacity
//                     for pk in providers_vec_from_db {
//                         let mut vecc = Vec::new();
//                         for i in &vec {
//                             if i.provider_kind == format!("{pk}") {
//                                 vecc.push(i.link_part.clone());
//                             }
//                         }
//                         hashmap_to_return.insert(pk, vecc);
//                     }
//                     Ok(hashmap_to_return)
//                 }
//             }
//         }
//     }
// }
