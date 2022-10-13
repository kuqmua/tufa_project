use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use impl_error_with_tracing_for_struct_without_get_source::ImplErrorWithTracingForStructWithoutGetSource;
use impl_get_source_without_method::ImplGetSourceWithoutMethod;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStruct;
use init_error::InitError;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use sqlx::Postgres;
use std::collections::HashMap;
use std::time::Duration;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    InitError,
    ImplGetSourceWithoutMethod,
    ImplGetWhereWasOneOrManyOneForErrorStruct,
    ImplErrorWithTracingForStructWithoutGetSource,
)]
pub struct PostgresEstablishConnectionError {
    pub source: Error,
    pub where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn postgres_establish_connection(
    providers_json_local_data_hashmap: &HashMap<ProviderKind, Vec<String>>,
    should_trace: bool,
) -> Result<sqlx::Pool<Postgres>, Box<PostgresEstablishConnectionError>> {
    match PgPoolOptions::new()
        .max_connections(providers_json_local_data_hashmap.len() as u32)
        .connect_timeout(Duration::from_millis(CONFIG.postgres_connection_timeout)) //todo add timeout constant or env var
        .connect(&CONFIG.get_postgres_url())
        .await
    {
        Err(e) => Err(Box::new(
            PostgresEstablishConnectionError::init_error_with_possible_trace(
                e,
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
        )),
        Ok(pool) => Ok(pool),
    }
}
