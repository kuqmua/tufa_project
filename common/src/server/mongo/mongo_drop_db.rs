#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)] //
pub enum MongoDropDbErrorNamed {
    MongoDB {
        #[eo_to_std_string_string]
        mongodb: mongodb::error::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub async fn mongo_drop_db<'a>(
    mongo_client: &mongodb::Client,
    db_name: &'a str,
) -> Result<(), Box<crate::server::mongo::mongo_drop_db::MongoDropDbErrorNamed>> {
    if let Err(error) = mongo_client.database(db_name).drop(None).await {
        return Err(Box::new(
            crate::server::mongo::mongo_drop_db::MongoDropDbErrorNamed::MongoDB {
                mongodb: error,
                code_occurence: error_occurence_lib::code_occurence!(),
            },
        ));
    }
    Ok(())
}

//////////////////
