#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum MongoGetDocumentsAsStringVectorErrorNamed {
    MongoDB {
        #[eo_to_std_string_string]
        mongodb: mongodb::error::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WrongBsonType {
        #[eo_to_std_string_string_serialize_deserialize]
        bson: mongodb::bson::Bson,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoKeyInDocument {
        #[eo_to_std_string_string_serialize_deserialize]
        key: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub async fn mongo_get_documents_as_string_vector(
    collection: mongodb::Collection<mongodb::bson::Document>,
    db_collection_document_field_name_handle: &str,
    option_aggregation: Option<mongodb::bson::Document>,
) -> Result<Vec<String>, Box<crate::server::mongo::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorErrorNamed>>{
    match collection.aggregate(option_aggregation, None).await {
        Err(error) => Err(Box::new(
            crate::server::mongo::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorErrorNamed::MongoDB {
                mongodb: error,
                code_occurence: error_occurence_lib::code_occurence!()
            }
        )),
        Ok(mut cursor) => {
            let mut vec_of_strings: Vec<String> = Vec::new();
            loop {
                match futures::stream::TryStreamExt::try_next(&mut cursor).await {
                    Err(error) => {
                        return Err(Box::new(
                            crate::server::mongo::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorErrorNamed::MongoDB {
                                mongodb: error,
                                code_occurence: error_occurence_lib::code_occurence!()
                            }
                        ));
                    }
                    Ok(option_document) => match option_document {
                        None => {
                            break;
                        }
                        Some(document) => {
                            match document.get(db_collection_document_field_name_handle) {
                                None => return Err(Box::new(
                                    crate::server::mongo::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorErrorNamed::NoKeyInDocument {
                                        key: db_collection_document_field_name_handle.to_string(),
                                        code_occurence: error_occurence_lib::code_occurence!()
                                    }
                                )),
                                Some(bson_handle) => match bson_handle {
                                    mongodb::bson::Bson::String(value) => {
                                        vec_of_strings.push(value.to_string());
                                    }
                                    other_bson_type => {
                                        return Err(Box::new(
                                            crate::server::mongo::mongo_get_documents_as_string_vector::MongoGetDocumentsAsStringVectorErrorNamed::WrongBsonType {
                                                bson: other_bson_type.clone(),
                                                code_occurence: error_occurence_lib::code_occurence!()
                                            }
                                    ));
                                    }
                                },
                            }
                        }
                    },
                }
            }
            Ok({
                use itertools::Itertools;
                vec_of_strings.into_iter().unique().collect()
            })
        }
    }
}
