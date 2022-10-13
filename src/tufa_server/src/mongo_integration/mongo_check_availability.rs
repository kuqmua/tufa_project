use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_error_with_tracing_for_struct_with_get_source_without_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWas;
use impl_get_source_with_method::ImplGetSourceWithMethod;
use impl_get_source_without_method::ImplGetSourceWithoutMethod;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStruct;
use init_error::InitError;
use mongodb::error::Error;
use mongodb::options::ClientOptions;
use mongodb::Client;
use std::time::Duration;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    ImplGetSourceWithMethod,
    ImplDisplayForErrorStruct,
    InitError,
    ImplErrorWithTracingForStructWithGetSourceWithoutGetWhereWas,
    ImplGetWhereWasOneOrManyOneForErrorStruct,
)]
pub struct MongoCheckAvailabilityError {
    source: MongoCheckAvailabilityErrorEnum,
    where_was: WhereWas,
}

#[derive(Debug, ImplGetSourceWithoutMethod, ImplDisplayForSimpleErrorEnum)]
pub enum MongoCheckAvailabilityErrorEnum {
    ClientOptionsParse(Error),
    ClientWithOptions(Error),
    ListCollectionNames(Error),
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_check_availability(
    mongo_url: &str,
    should_trace: bool,
) -> Result<(), Box<MongoCheckAvailabilityError>> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(Box::new(
            MongoCheckAvailabilityError::init_error_with_possible_trace(
                MongoCheckAvailabilityErrorEnum::ClientOptionsParse(e),
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
        Ok(mut client_options) => {
            client_options.connect_timeout =
                Some(Duration::from_millis(CONFIG.mongo_connection_timeout));
            match Client::with_options(client_options) {
                Err(e) => Err(Box::new(
                    MongoCheckAvailabilityError::init_error_with_possible_trace(
                        MongoCheckAvailabilityErrorEnum::ClientWithOptions(e),
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
                Ok(client) => {
                    if let Err(e) = client
                        .database(&CONFIG.mongo_providers_logs_db_name)
                        .list_collection_names(None)
                        .await
                    {
                        return Err(Box::new(
                            MongoCheckAvailabilityError::init_error_with_possible_trace(
                                MongoCheckAvailabilityErrorEnum::ListCollectionNames(e),
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
            }
        }
    }
}
