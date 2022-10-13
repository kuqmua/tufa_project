use super::mongo_insert_docs_in_empty_collection::MongoInsertDocsInEmptyCollectionErrorEnum;
use crate::lazy_static::config::CONFIG;
use crate::mongo_integration::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::future::join_all;
use git_info::GitInfo;
use std::collections::HashMap;
use tufa_common::where_was::WhereWas;

#[derive(Debug, GitInfo)]
pub struct MongoInsertDataError {
    pub source: HashMap<ProviderKind, MongoInsertDocsInEmptyCollectionErrorEnum>,
    where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_insert_data(
    db_name_handle: &str,
    vec_of_link_parts_hashmap: HashMap<ProviderKind, Vec<String>>,
) -> Result<(), Box<MongoInsertDataError>> {
    let error_hashmap = join_all(vec_of_link_parts_hashmap.into_iter().map(
        |(pk, vec_of_link_parts)| async move {
            (
                pk,
                mongo_insert_docs_in_empty_collection(
                    db_name_handle,
                    format!(
                        "{pk}{}",
                        CONFIG.mongo_providers_logs_db_collection_handle_second_part
                    ),
                    vec_of_link_parts,
                )
                .await,
            )
        },
    ))
    .await
    .into_iter()
    .filter_map(|(pk, result)| {
        if let Err(e) = result {
            return Some((pk, *e));
        }
        None
    })
    .collect::<HashMap<ProviderKind, MongoInsertDocsInEmptyCollectionErrorEnum>>();
    if !error_hashmap.is_empty() {
        return Err(Box::new(MongoInsertDataError {
            source: error_hashmap,
            where_was: WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
        }));
    }
    Ok(())
}
