use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::mongo_integration::mongo_check_collection_is_not_empty::mongo_check_collections_is_not_empty;
use crate::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionsIsNotEmptyError;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::future::join_all;
use impl_error_with_tracing_for_struct_without_get_source::ImplErrorWithTracingForStructWithoutGetSource;
use impl_get_source_with_method::ImplGetSourceWithMethod;
use impl_get_source_without_method::ImplGetSourceWithoutMethod;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStruct;
use init_error::InitError;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::error::Error;
use mongodb::options::ClientOptions;
use mongodb::Client;
use std::collections::HashMap;
use tufa_common::traits::get_source::GetSource;
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
pub struct MongoClientOptionsParseError {
    source: Error,
    where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_client_options_parse(
    should_trace: bool,
) -> Result<ClientOptions, Box<MongoClientOptionsParseError>> {
    match ClientOptions::parse(&CONFIG.get_mongo_url()).await {
        Err(e) => Err(Box::new(
            MongoClientOptionsParseError::init_error_with_possible_trace(
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
        Ok(client_options) => Ok(client_options),
    }
}
