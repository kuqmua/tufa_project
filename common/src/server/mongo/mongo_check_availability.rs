#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum MongoCheckAvailabilityErrorNamed {
    ListCollectionNames {
        #[eo_display]
        list_collection_names: mongodb::error::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub async fn mongo_check_availability<'a>(
    mongo_client: &mongodb::Client,
    db_name: &str,
) -> Result<(), Box<MongoCheckAvailabilityErrorNamed>> {
    if let Err(e) = mongo_client
        .database(db_name)
        .list_collection_names(None)
        .await
    {
        return Err(Box::new(
            MongoCheckAvailabilityErrorNamed::ListCollectionNames {
                list_collection_names: e,
                code_occurence: crate::code_occurence!(),
            },
        ));
    }
    Ok(())
}
