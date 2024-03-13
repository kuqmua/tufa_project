#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum WriteJsonIntoFileAsyncTokioErrorNamed {
    SerdeJson {
        #[eo_display]
        serde_json_error: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WriteBytesIntoFile {
        #[eo_error_occurence] 
        write_bytes_into_file: crate::server::file_system::write_bytes_into_file::write_bytes_into_file_async_tokio::WriteBytesIntoFileAsyncTokioErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    }, 
}

pub async fn write_json_into_file_async_tokio(
    path: &std::path::Path,
    json_object: serde_json::Value,
) -> Result<(), Box<WriteJsonIntoFileAsyncTokioErrorNamed>> {
    match serde_json::to_string_pretty(&json_object) {
        Err(e) => {
            Err(Box::new(
                WriteJsonIntoFileAsyncTokioErrorNamed::SerdeJson {
                    serde_json_error: e, 
                    code_occurence: error_occurence_lib::code_occurence!() 
                },
            ))
        }
        Ok(stringified_json) => {
            match crate::server::file_system::write_bytes_into_file::write_bytes_into_file_async_tokio::write_bytes_into_file_async_tokio(
                path,
                stringified_json.as_bytes(),
            )
            .await {
                Err(e) => {
                    Err(Box::new(
                        WriteJsonIntoFileAsyncTokioErrorNamed::WriteBytesIntoFile {
                            write_bytes_into_file: *e, 
                            code_occurence: error_occurence_lib::code_occurence!() 
                        },
                    ))
                },
                Ok(_) => Ok(())
            }
        },
    }
}
///////////////////