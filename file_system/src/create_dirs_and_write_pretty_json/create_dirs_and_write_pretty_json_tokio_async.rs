#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum CreateDirsAndWritePrettyJsonTokioAsyncErrorNamed {
    SerdeJson {
        #[eo_to_std_string_string]
        error: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WriteBytesIntoFile {
        #[eo_error_occurence] 
        error: crate::CreateDirsAndWriteFileTokioAsyncErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    }, 
}

pub async fn create_dirs_and_write_pretty_json_tokio_async(path: &std::path::Path, serde_json_value: serde_json::Value) -> Result<(), CreateDirsAndWritePrettyJsonTokioAsyncErrorNamed> {
    match serde_json::to_string_pretty(&serde_json_value) {
        Ok(value) => match crate::create_dirs_and_write_file_tokio_async(
            path,
            value.as_bytes(),
        )
        .await {
            Err(error) => Err(CreateDirsAndWritePrettyJsonTokioAsyncErrorNamed::WriteBytesIntoFile {
                error,
                code_occurence: error_occurence_lib::code_occurence!() 
            }),
            Ok(()) => Ok(())
        },
        Err(error) => Err(CreateDirsAndWritePrettyJsonTokioAsyncErrorNamed::SerdeJson {
            error,
            code_occurence: error_occurence_lib::code_occurence!() 
        })
    }
}