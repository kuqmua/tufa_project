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
pub struct PostgresInsertLinkPartsIntoProvidersTablesError {
    source: HashMap<ProviderKind, sqlx::Error>,
    where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn postgres_insert_link_parts_into_providers_tables(
    providers_json_local_data_hashmap: &HashMap<ProviderKind, Vec<String>>,
    pool: &Pool<Postgres>,
    should_trace: bool,
) -> Result<(), Box<PostgresInsertLinkPartsIntoProvidersTablesError>> {
    let insertion_error_hashmap = join_all(providers_json_local_data_hashmap.iter().map(
        |(pk, string_vec)| async {
            let mut values_string = String::from("");
            for link_part in string_vec.clone() {
                values_string.push_str(&format!("('{link_part}'),"));
            }
            if !values_string.is_empty() {
                values_string.pop();
            }
            let query_string = format!(
                "INSERT INTO {} (link_part) VALUES {values_string};",
                pk.get_postgres_table_name()
            );
            (*pk, sqlx::query(&query_string).execute(pool).await)
        },
    ))
    .await
    .into_iter()
    .filter_map(|(pk, result)| {
        if let Err(e) = result {
            return Some((pk, e));
        }
        None
    })
    .collect::<HashMap<ProviderKind, sqlx::Error>>();
    if !insertion_error_hashmap.is_empty() {
        return Err(Box::new(
            PostgresInsertLinkPartsIntoProvidersTablesError::init_error_with_possible_trace(
                insertion_error_hashmap,
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
