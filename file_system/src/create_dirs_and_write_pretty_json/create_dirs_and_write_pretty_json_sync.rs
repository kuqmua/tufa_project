use error_occurence_lib::{ErrorOccurence, code_occurence, code_occurence::CodeOccurence};
use serde_json::{Error as SerdeJsonError, Value as SerdeJsonValue, to_string_pretty};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error, ErrorOccurence)]
pub enum CreateDirsAndWritePrettyJsonSyncError {
    SerdeJson {
        #[eo_to_err_string]
        error: SerdeJsonError,
        code_occurence: CodeOccurence,
    },
    WriteBytesIntoFile {
        #[eo_error_occurence]
        error: crate::CreateDirsAndWriteFileSyncError,
        code_occurence: CodeOccurence,
    },
}

pub fn create_dirs_and_write_pretty_json_sync(
    path: &Path,
    serde_json_value: &SerdeJsonValue,
) -> Result<(), CreateDirsAndWritePrettyJsonSyncError> {
    match to_string_pretty(&serde_json_value) {
        Ok(value) => {
            if let Err(error) = crate::create_dirs_and_write_file_sync(path, value.as_bytes()) {
                return Err(CreateDirsAndWritePrettyJsonSyncError::WriteBytesIntoFile {
                    error,
                    code_occurence: code_occurence!(),
                });
            }
            Ok(())
        }
        Err(error) => Err(CreateDirsAndWritePrettyJsonSyncError::SerdeJson {
            error,
            code_occurence: code_occurence!(),
        }),
    }
}
