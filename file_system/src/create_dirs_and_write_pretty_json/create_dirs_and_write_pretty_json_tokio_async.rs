use error_occurence_lib::{ErrorOccurence, code_occurence, code_occurence::CodeOccurence};
use serde_json::{Error as SerdeJsonError, Value as SerdeJsonValue, to_string_pretty};
use std::path::Path;
use thiserror::Error;
#[derive(Debug, Error, ErrorOccurence)]
pub enum CreateDirsAndWritePrettyJsonTokioAsyncError {
    SerdeJson {
        #[eo_to_err_string]
        er: SerdeJsonError,
        code_occurence: CodeOccurence,
    },
    WriteBytesIntoFile {
        #[eo_error_occurence]
        er: crate::CreateDirsAndWriteFileTokioAsyncError,
        code_occurence: CodeOccurence,
    },
}
pub async fn create_dirs_and_write_pretty_json_tokio_async(
    path: &Path,
    serde_json_value: SerdeJsonValue,
) -> Result<(), CreateDirsAndWritePrettyJsonTokioAsyncError> {
    match to_string_pretty(&serde_json_value) {
        Ok(value) => {
            match crate::create_dirs_and_write_file_tokio_async(path, value.as_bytes()).await {
                Err(er) => Err(
                    CreateDirsAndWritePrettyJsonTokioAsyncError::WriteBytesIntoFile {
                        er,
                        code_occurence: code_occurence!(),
                    },
                ),
                Ok(()) => Ok(()),
            }
        }
        Err(er) => Err(CreateDirsAndWritePrettyJsonTokioAsyncError::SerdeJson {
            er,
            code_occurence: code_occurence!(),
        }),
    }
}
