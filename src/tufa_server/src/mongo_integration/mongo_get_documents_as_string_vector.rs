use crate::lazy_static::config::CONFIG;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::stream::TryStreamExt;
use itertools::Itertools;
use mongodb::bson::Document;
use mongodb::Collection;
use tufa_common::where_was::WhereWas;

#[derive(Debug)]
pub enum MongoGetDocumentsAsStringVectorErrorEnum {
    CollectionAggregate {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    CursorTryNext {
        source: mongodb::error::Error,
        where_was: WhereWas,
    },
    WrongBsonType {
        source: mongodb::bson::Bson,
        where_was: WhereWas,
    },
    NoKeyInDocument {
        source: String,
        where_was: WhereWas,
    },
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_get_documents_as_string_vector(
    collection: Collection<Document>,
    db_collection_document_field_name_handle: &str,
    option_aggregation: Option<Document>,
) -> Result<Vec<String>, Box<MongoGetDocumentsAsStringVectorErrorEnum>> {
    match collection.aggregate(option_aggregation, None).await {
        Err(e) => Err(Box::new(
            MongoGetDocumentsAsStringVectorErrorEnum::CollectionAggregate {
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
        Ok(mut cursor) => {
            let mut vec_of_strings: Vec<String> = Vec::new();
            loop {
                match cursor.try_next().await {
                    Err(e) => {
                        return Err(Box::new(
                            MongoGetDocumentsAsStringVectorErrorEnum::CursorTryNext {
                                source: e,
                                where_was: WhereWas {
                                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                                    file: file!(),
                                    line: line!(),
                                    column: column!(),
                                },
                            },
                        ));
                    }
                    Ok(option_document) => {
                        match option_document {
                            None => {
                                break;
                            }
                            Some(document) => {
                                match document.get(db_collection_document_field_name_handle) {
                                    None => return Err(Box::new(
                                        MongoGetDocumentsAsStringVectorErrorEnum::NoKeyInDocument {
                                            source: db_collection_document_field_name_handle
                                                .to_string(),
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
                                    )),
                                    Some(bson_handle) => match bson_handle {
                                        mongodb::bson::Bson::String(value) => {
                                            vec_of_strings.push(value.to_string());
                                        }
                                        other_bson_type => {
                                            return Err(Box::new(
                                            MongoGetDocumentsAsStringVectorErrorEnum::WrongBsonType {
                                                source: other_bson_type.clone(),
                where_was: WhereWas {
                time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                    .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                file: file!(),
                line: line!(),
                column: column!(),
            },
                                            },
                                        ));
                                        }
                                    },
                                }
                            }
                        }
                    }
                }
            }
            Ok(vec_of_strings.into_iter().unique().collect())
        }
    }
}
