use crate::lazy_static::config::CONFIG;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use mongodb::bson::Document;
use mongodb::options::ClientOptions;
use mongodb::Client;
use tufa_common::where_was::WhereWas;

#[derive(Debug)]
pub enum MongoCheckCollectionIsEmptyErrorEnum {
    ClientOptionsParse {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    ClientWithOptions {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    CountDocuments {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    NotEmpty {
        source: u64,
        where_was: WhereWas,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_check_collection_is_empty(
    mongo_url: &str,
    db_name: &str,
    db_collection_name: &str,
) -> Result<(), Box<MongoCheckCollectionIsEmptyErrorEnum>> {
    match ClientOptions::parse(mongo_url).await {
        Err(e) => Err(Box::new(
            MongoCheckCollectionIsEmptyErrorEnum::ClientOptionsParse {
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
        Ok(client_options) => match Client::with_options(client_options) {
            Err(e) => Err(Box::new(
                MongoCheckCollectionIsEmptyErrorEnum::ClientWithOptions {
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
            Ok(client) => {
                match client
                    .database(db_name)
                    .collection::<Document>(db_collection_name)
                    .count_documents(None, None)
                    .await
                {
                    Err(e) => Err(Box::new(
                        MongoCheckCollectionIsEmptyErrorEnum::CountDocuments {
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
                        if documents_number > 0 {
                            return Err(Box::new(MongoCheckCollectionIsEmptyErrorEnum::NotEmpty {
                                source: documents_number,
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
                }
            }
        },
    }
}
