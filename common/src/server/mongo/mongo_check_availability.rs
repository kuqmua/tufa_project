#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurenceTest)]
pub enum MongoCheckAvailabilityErrorNamed {
    ListCollectionNames {
        #[eo_to_std_string_string]
        list_collection_names: mongodb::error::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub async fn mongo_check_availability<'a>(
    mongo_client: &mongodb::Client,
    db_name: &str,
) -> Result<(), Box<MongoCheckAvailabilityErrorNamed>> {
    if let Err(error) = mongo_client
        .database(db_name)
        .list_collection_names(None)
        .await
    {
        return Err(Box::new(
            MongoCheckAvailabilityErrorNamed::ListCollectionNames {
                list_collection_names: error,
                code_occurence: error_occurence_lib::code_occurence!(),
            },
        ));
    }
    Ok(())
}
