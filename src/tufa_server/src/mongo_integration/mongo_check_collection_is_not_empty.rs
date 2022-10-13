use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::future::join_all;
use impl_error_with_tracing_for_struct_with_get_source_without_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWas;
use impl_get_source_with_method::ImplGetSourceWithMethod;
use impl_get_source_without_method::ImplGetSourceWithoutMethod;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStruct;
use init_error::InitError;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::error::Error;
use mongodb::options::ClientOptions;
use mongodb::Client;
use mongodb::Database;
use std::collections::HashMap;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    InitError,
    ImplGetSourceWithMethod,
    ImplGetWhereWasOneOrManyOneForErrorStruct,
    ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWas,
)]
pub struct MongoCheckCollectionsIsNotEmptyError {
    source: HashMap<ProviderKind, CollectionCountDocumentsOrIsNotEmpty>,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceWithoutMethod)]
pub enum CollectionCountDocumentsOrIsNotEmpty {
    CountDocuments(Error),
    IsNotEmpty(u64),
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_check_collections_is_not_empty(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
    db: &Database,
    should_trace: bool,
) -> Result<(), Box<MongoCheckCollectionsIsNotEmptyError>> {
    let error_vec_count_documents =
        join_all(providers_json_local_data_hashmap.keys().map(|pk| async {
            (
                *pk,
                db.collection::<Document>(&pk.get_db_tag())
                    .count_documents(None, None)
                    .await,
            )
        }))
        .await
        .into_iter()
        .filter_map(|(pk, result)| match result {
            Err(e) => Some((pk, CollectionCountDocumentsOrIsNotEmpty::CountDocuments(e))),
            Ok(documents_number) => {
                if documents_number > 0 {
                    return Some((
                        pk,
                        CollectionCountDocumentsOrIsNotEmpty::IsNotEmpty(documents_number),
                    ));
                }
                None
            }
        })
        .collect::<HashMap<ProviderKind, CollectionCountDocumentsOrIsNotEmpty>>();
    if !error_vec_count_documents.is_empty() {
        return Err(Box::new(
            MongoCheckCollectionsIsNotEmptyError::init_error_with_possible_trace(
                error_vec_count_documents,
                WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO.data,
                should_trace,
            ),
        ));
    }
    Ok(())
}
