use crate::lazy_static::config::CONFIG;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use mongodb::options::ClientOptions;
use mongodb::Client;
use tufa_common::where_was::WhereWas;

#[derive(Debug)]
pub enum MongoCheckDbIsEmptyErrorEnum {
    ClientOptionsParse {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    ClientWithOptions {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    ListCollectionNames {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    NotEmpty {
        source: usize,
        where_was: WhereWas,
    },
    DatabaseDrop {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_check_db_is_empty(
    mongo_url: &str,
    db_name: &str,
) -> Result<(), Box<MongoCheckDbIsEmptyErrorEnum>> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(Box::new(MongoCheckDbIsEmptyErrorEnum::ClientOptionsParse {
            source: e,
            where_was: WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
        })),
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => Err(Box::new(MongoCheckDbIsEmptyErrorEnum::ClientWithOptions {
                source: e,
                where_was: WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            })),
            Ok(client) => match client.database(db_name).list_collection_names(None).await {
                Err(e) => Err(Box::new(
                    MongoCheckDbIsEmptyErrorEnum::ListCollectionNames {
                        source: e,
                        where_was: WhereWas {
                            time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    },
                )),
                Ok(documents_number) => {
                    if !documents_number.is_empty() {
                        return Err(Box::new(MongoCheckDbIsEmptyErrorEnum::NotEmpty {
                            source: documents_number.len(),
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
            },
        },
    }
}
