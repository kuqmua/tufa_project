use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::mongo_integration::mongo_check_availability::mongo_check_availability;
use crate::mongo_integration::mongo_check_availability::MongoCheckAvailabilityError;
use crate::net_check::net_check_availability::net_check_availability;
use crate::net_check::net_check_availability::NetCheckAvailabilityError;
use crate::postgres_integration::postgres_check_availability::postgres_check_availability;
use crate::postgres_integration::postgres_check_availability::PostgresCheckAvailabilityError;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::join;
use impl_display_for_error_struct::ImplDisplayForErrorStruct;
use impl_display_for_simple_error_enum::ImplDisplayForSimpleErrorEnum;
use impl_error_with_tracing_for_struct_with_get_source_with_get_where_was::ImplErrorWithTracingForStructWithGetSourceWithGetWhereWas;
use impl_get_source_with_method::ImplGetSourceWithMethod;
use impl_get_source_without_method::ImplGetSourceWithoutMethod;
use impl_get_where_was_one_or_many_with_method::ImplGetWhereWasOneOrManyWithMethod;
use init_error::InitError;
use tufa_common::traits::get_log_with_additional_where_was::GetLogWithAdditionalWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;

#[derive(
    Debug,
    ImplGetSourceWithMethod,
    ImplDisplayForErrorStruct,
    InitError,
    ImplErrorWithTracingForStructWithGetSourceWithGetWhereWas,
    ImplGetWhereWasOneOrManyWithMethod,
)]
pub struct CheckAvailabilityError {
    source: CheckAvailabilityErrorEnum,
    where_was: WhereWas,
}

#[derive(
    Debug,
    ImplGetSourceWithoutMethod,
    ImplDisplayForSimpleErrorEnum,
    ImplGetWhereWasOneOrManyWithMethod,
)]
pub enum CheckAvailabilityErrorEnum {
    Net(Box<NetCheckAvailabilityError>),
    Postgres(Box<PostgresCheckAvailabilityError>),
    Mongo(Box<MongoCheckAvailabilityError>),
    NetAndMongo {
        net_source: Box<NetCheckAvailabilityError>,
        mongo_source: Box<MongoCheckAvailabilityError>,
    },
    NetAndPostgres {
        net_source: Box<NetCheckAvailabilityError>,
        postgres_source: Box<PostgresCheckAvailabilityError>,
    },
    MongoAndPostgres {
        mongo_source: Box<MongoCheckAvailabilityError>,
        postgres_source: Box<PostgresCheckAvailabilityError>,
    },
    NetAndMongoAndPostgres {
        net_source: Box<NetCheckAvailabilityError>,
        mongo_source: Box<MongoCheckAvailabilityError>,
        postgres_source: Box<PostgresCheckAvailabilityError>,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn check_availability(should_trace: bool) -> Result<(), Box<CheckAvailabilityError>> {
    let net_url = &CONFIG.starting_check_link.clone();
    let postgres_url = &CONFIG.get_postgres_url();
    let mongo_url = &CONFIG.get_mongo_url();
    match join!(
        net_check_availability(net_url, false),
        postgres_check_availability(postgres_url, false),
        mongo_check_availability(mongo_url, false),
    ) {
        (Ok(_), Ok(_), Ok(_)) => Ok(()),
        (Ok(_), Ok(_), Err(m)) => Err(Box::new(
            CheckAvailabilityError::init_error_with_possible_trace(
                CheckAvailabilityErrorEnum::Mongo(m),
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
        (Ok(_), Err(p), Ok(_)) => Err(Box::new(
            CheckAvailabilityError::init_error_with_possible_trace(
                CheckAvailabilityErrorEnum::Postgres(p),
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
        (Ok(_), Err(p), Err(m)) => Err(Box::new(
            CheckAvailabilityError::init_error_with_possible_trace(
                CheckAvailabilityErrorEnum::MongoAndPostgres {
                    mongo_source: m,
                    postgres_source: p,
                },
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
        (Err(n), Ok(_), Ok(_)) => Err(Box::new(
            CheckAvailabilityError::init_error_with_possible_trace(
                CheckAvailabilityErrorEnum::Net(n),
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
        (Err(n), Ok(_), Err(m)) => Err(Box::new(
            CheckAvailabilityError::init_error_with_possible_trace(
                CheckAvailabilityErrorEnum::NetAndMongo {
                    net_source: n,
                    mongo_source: m,
                },
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
        (Err(n), Err(p), Ok(_)) => Err(Box::new(
            CheckAvailabilityError::init_error_with_possible_trace(
                CheckAvailabilityErrorEnum::NetAndPostgres {
                    net_source: n,
                    postgres_source: p,
                },
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
        (Err(n), Err(p), Err(m)) => Err(Box::new(
            CheckAvailabilityError::init_error_with_possible_trace(
                CheckAvailabilityErrorEnum::NetAndMongoAndPostgres {
                    net_source: n,
                    postgres_source: p,
                    mongo_source: m,
                },
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
    }
}
