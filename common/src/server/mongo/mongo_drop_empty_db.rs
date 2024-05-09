#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum MongoDropEmptyDbErrorNamed {
    MongoDB {
        #[eo_to_std_string_string]
        mongodb: mongodb::error::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CollectionNamesListIsNotEmpty {
        #[eo_to_std_string_string_serialize_deserialize]
        database: std::string::String, //todo conversion to_string() for with_serialize_deserialize
        #[eo_to_std_string_string_serialize_deserialize]
        list_collection_names_len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub async fn mongo_drop_empty_db<'a>(
    mongo_client: &mongodb::Client,
    db_name: &'a str,
) -> Result<(), Box<crate::server::mongo::mongo_drop_empty_db::MongoDropEmptyDbErrorNamed>> {
    let db = mongo_client.database(db_name);
    match db.list_collection_names(None).await {
        Err(error) => Err(Box::new(
            crate::server::mongo::mongo_drop_empty_db::MongoDropEmptyDbErrorNamed::MongoDB {
                mongodb: error,
                code_occurence: error_occurence_lib::code_occurence!(),
            },
        )),
        Ok(collections_names_list) => {
            if !collections_names_list.is_empty() {
                return Err(Box::new(
                    crate::server::mongo::mongo_drop_empty_db::MongoDropEmptyDbErrorNamed::CollectionNamesListIsNotEmpty {
                        database: db_name.to_string(),
                        list_collection_names_len: collections_names_list.len(),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }
                ));
            }
            if let Err(error) = db.drop(None).await {
                return Err(Box::new(
                    crate::server::mongo::mongo_drop_empty_db::MongoDropEmptyDbErrorNamed::MongoDB {
                        mongodb: error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }
                ));
            }
            Ok(())
        }
    }
}
