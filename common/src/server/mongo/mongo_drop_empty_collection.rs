#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
pub enum MongoDropEmptyCollectionErrorNamed {
    MongoDB {
        #[eo_to_std_string_string]
        mongodb: mongodb::error::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CollectionIsNotEmpty {
        #[eo_to_std_string_string_serialize_deserialize]
        collection_name: std::string::String,
        #[eo_to_std_string_string_serialize_deserialize]
        collection_len: u64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub async fn mongo_drop_empty_collection<'a>(
    mongo_client: &mongodb::Client,
    db_name: &'a str,
    db_collection_name: std::string::String,
) -> Result<
    (),
    Box<crate::server::mongo::mongo_drop_empty_collection::MongoDropEmptyCollectionErrorNamed>,
> {
    let collection: mongodb::Collection<mongodb::bson::Document> = mongo_client
        .database(db_name)
        .collection(&db_collection_name);
    match 
        collection
        .count_documents(None, None)
        .await 
    {
        Err(error) => Err(Box::new(
            crate::server::mongo::mongo_drop_empty_collection::MongoDropEmptyCollectionErrorNamed::MongoDB {
                mongodb: error,
                code_occurence: error_occurence_lib::code_occurence!(),
            }
        )),
        Ok(documents_number) => {
            if documents_number > 0 {
                Err(Box::new(
                    crate::server::mongo::mongo_drop_empty_collection::MongoDropEmptyCollectionErrorNamed::CollectionIsNotEmpty {
                        collection_name: db_collection_name,
                        collection_len: documents_number,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }
                ))
            } else {
                if let Err(error) = collection.drop(None).await {
                    return Err(Box::new(
                        crate::server::mongo::mongo_drop_empty_collection::MongoDropEmptyCollectionErrorNamed::MongoDB {
                            mongodb: error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        }
                    ));
                }
                Ok(())
            }
        }
    }
}
