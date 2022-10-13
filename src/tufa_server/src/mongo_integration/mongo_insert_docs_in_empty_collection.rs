use crate::lazy_static::config::CONFIG;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::options::ClientOptions;
use mongodb::Client;
use tufa_common::where_was::WhereWas;

#[derive(Debug)]
pub enum MongoInsertDocsInEmptyCollectionErrorEnum {
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
    CollectionInsertMany {
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
pub async fn mongo_insert_docs_in_empty_collection(
    db_name_handle: &str,
    db_collection_handle: String,
    vec_of_values: Vec<String>,
) -> Result<(), Box<MongoInsertDocsInEmptyCollectionErrorEnum>> {
    match ClientOptions::parse(CONFIG.get_mongo_url()).await {
        Err(e) => Err(Box::new(
            MongoInsertDocsInEmptyCollectionErrorEnum::ClientOptionsParse {
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
                MongoInsertDocsInEmptyCollectionErrorEnum::ClientWithOptions {
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
                let collection = client
                    .database(db_name_handle)
                    .collection(&db_collection_handle);
                match collection.count_documents(None, None).await {
                    Err(e) => Err(Box::new(
                        MongoInsertDocsInEmptyCollectionErrorEnum::CountDocuments {
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
                            Err(Box::new(
                                MongoInsertDocsInEmptyCollectionErrorEnum::NotEmpty {
                                    source: documents_number,
                                    where_was: WhereWas {
                                        time: DateTime::<Utc>::from_utc(
                                            Local::now().naive_utc(),
                                            Utc,
                                        )
                                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                        file: file!(),
                                        line: line!(),
                                        column: column!(),
                                    },
                                },
                            ))
                        } else {
                            if let Err(e) = collection.insert_many(
                                vec_of_values.iter()
                                .map(|value|doc! { &CONFIG.mongo_providers_logs_db_collection_document_field_name_handle: value })
                                .collect::<Vec<Document>>(),
                                None).await {
                                return Err(
                                    Box::new(MongoInsertDocsInEmptyCollectionErrorEnum::CollectionInsertMany {
                                source: e,
                                             where_was: WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
                            }),
                            );
                            }
                            Ok(())
                        }
                    }
                }
            }
        },
    }
}
