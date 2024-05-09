#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
pub enum MongoCheckCollectionIsEmptyErrorNamed {
    MongoDB {
        #[eo_to_std_string_string]
        mongodb: mongodb::error::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CollectionIsNotEmpty {
        #[eo_to_std_string_string_serialize_deserialize]
        collection_documents: u64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub async fn mongo_check_collection_is_empty<'a>(
    mongo_client: &mongodb::Client,
    db_name: &str,
    db_collection_name: &str,
) -> Result<(), Box<MongoCheckCollectionIsEmptyErrorNamed>> {
    match mongo_client
        .database(db_name)
        .collection::<mongodb::bson::Document>(db_collection_name)
        .count_documents(None, None)
        .await
    {
        Err(error) => Err(Box::new(MongoCheckCollectionIsEmptyErrorNamed::MongoDB {
            mongodb: error,
            code_occurence: error_occurence_lib::code_occurence!(),
        })),
        Ok(documents_number) => {
            if documents_number > 0 {
                return Err(Box::new(
                    MongoCheckCollectionIsEmptyErrorNamed::CollectionIsNotEmpty {
                        collection_documents: documents_number,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    },
                ));
            }
            Ok(())
        }
    }
}
