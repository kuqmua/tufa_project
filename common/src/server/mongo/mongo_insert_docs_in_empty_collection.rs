#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum MongoInsertDocsInEmptyCollectionErrorNamed {
    MongoDB {
        #[eo_to_std_string_string]
        mongodb: mongodb::error::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CollectionIsNotEmpty {
        #[eo_to_std_string_string_serialize_deserialize]
        collection_is_not_empty: u64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub async fn mongo_insert_docs_in_empty_collection<'a>(
    mongo_client: &mongodb::Client,
    db_name_handle: &str,
    db_collection_handle: std::string::String,
    collection_field_name: &'a std::string::String,
    vec_of_values: Vec<String>,
) -> Result<(), Box<MongoInsertDocsInEmptyCollectionErrorNamed>> {
    let collection = mongo_client
        .database(db_name_handle)
        .collection(&db_collection_handle);
    match collection.count_documents(None, None).await {
        Err(error) => Err(Box::new(
            MongoInsertDocsInEmptyCollectionErrorNamed::MongoDB {
                mongodb: error,
                code_occurence: error_occurence_lib::code_occurence!(),
            },
        )),
        Ok(documents_number) => {
            if documents_number > 0 {
                Err(Box::new(
                    MongoInsertDocsInEmptyCollectionErrorNamed::CollectionIsNotEmpty {
                        collection_is_not_empty: documents_number,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    },
                ))
            } else {
                if let Err(error) = collection
                    .insert_many(
                        vec_of_values
                            .iter()
                            .map(|value| mongodb::bson::doc! { collection_field_name: value })
                            .collect::<Vec<mongodb::bson::Document>>(),
                        None,
                    )
                    .await
                {
                    return Err(Box::new(
                        MongoInsertDocsInEmptyCollectionErrorNamed::MongoDB {
                            mongodb: error,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        },
                    ));
                }
                Ok(())
            }
        }
    }
}
