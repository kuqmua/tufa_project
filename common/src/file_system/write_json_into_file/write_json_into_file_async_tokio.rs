#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum WriteJsonIntoFileAsyncTokioErrorNamed {
    SerdeJson {
        #[eo_to_std_string_string]
        error: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WriteBytesIntoFile {
        #[eo_error_occurence] 
        error: crate::file_system::write_bytes_into_file::write_bytes_into_file_async_tokio::WriteBytesIntoFileAsyncTokioErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    }, 
}

pub async fn write_json_into_file_async_tokio(path: &std::path::Path, json_object: serde_json::Value) -> Result<(), WriteJsonIntoFileAsyncTokioErrorNamed> {
    match serde_json::to_string_pretty(&json_object) {
        Ok(stringified_json) => match crate::file_system::write_bytes_into_file::write_bytes_into_file_async_tokio::write_bytes_into_file_async_tokio(
            path,
            stringified_json.as_bytes(),
        )
        .await {
            Err(error) => Err(WriteJsonIntoFileAsyncTokioErrorNamed::WriteBytesIntoFile {
                error,
                code_occurence: error_occurence_lib::code_occurence!() 
            }),
            Ok(()) => Ok(())
        },
        Err(error) => Err(WriteJsonIntoFileAsyncTokioErrorNamed::SerdeJson {
            error,
            code_occurence: error_occurence_lib::code_occurence!() 
        })
    }
}