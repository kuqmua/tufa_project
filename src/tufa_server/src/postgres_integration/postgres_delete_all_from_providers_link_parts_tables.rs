use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::future::join_all;
use impl_error_with_tracing_for_struct_without_get_source::ImplErrorWithTracingForStructWithoutGetSource;
use impl_get_source_without_method::ImplGetSourceWithoutMethod;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStruct;
use init_error::InitError;
use sqlx::Pool;
use sqlx::Postgres;
use std::collections::HashMap;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    InitError,
    ImplGetWhereWasOneOrManyOneForErrorStruct,
    ImplGetSourceWithoutMethod,
    ImplErrorWithTracingForStructWithoutGetSource,
)]
pub struct PostgresDeleteAllFromProvidersTablesError {
    pub source: HashMap<ProviderKind, sqlx::Error>,
    pub where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn postgres_delete_all_from_providers_link_parts_tables(
    providers_json_local_data_hashmap: &HashMap<ProviderKind, Vec<String>>,
    pool: &Pool<Postgres>,
    should_trace: bool,
) -> Result<(), Box<PostgresDeleteAllFromProvidersTablesError>> {
    let delete_from_tables_error_hashmap =
        join_all(providers_json_local_data_hashmap.keys().map(|pk| async {
            let query_string = format!("DELETE FROM {} ;", pk.get_postgres_table_name());
            (*pk, sqlx::query(&query_string).execute(pool).await)
        }))
        .await
        .into_iter()
        .filter_map(|(pk, result)| {
            if let Err(e) = result {
                return Some((pk, e));
            }
            None
        })
        .collect::<HashMap<ProviderKind, sqlx::Error>>();
    if !delete_from_tables_error_hashmap.is_empty() {
        return Err(Box::new(
            PostgresDeleteAllFromProvidersTablesError::init_error_with_possible_trace(
                delete_from_tables_error_hashmap,
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
