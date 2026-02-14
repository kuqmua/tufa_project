use error_occurence_lib::code_occurence::CodeOccurence;
use serde_json::{Error as SerdeJsonError, Value as SerdeJsonValue};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error, error_occurence_lib::ErrorOccurence)]
pub enum CreateDirsAndWritePrettyJsonSyncErrorNamed {
    SerdeJson {
        #[eo_to_err_string]
        error: SerdeJsonError,
        code_occurence: CodeOccurence,
    },
    WriteBytesIntoFile {
        #[eo_error_occurence]
        error: crate::CreateDirsAndWriteFileSyncErrorNamed,
        code_occurence: CodeOccurence,
    },
}

pub fn create_dirs_and_write_pretty_json_sync(
    path: &Path,
    serde_json_value: &SerdeJsonValue,
) -> Result<(), CreateDirsAndWritePrettyJsonSyncErrorNamed> {
    match serde_json::to_string_pretty(&serde_json_value) {
        Ok(value) => {
            if let Err(error) = crate::create_dirs_and_write_file_sync(path, value.as_bytes()) {
                return Err(
                    CreateDirsAndWritePrettyJsonSyncErrorNamed::WriteBytesIntoFile {
                        error,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    },
                );
            }
            Ok(())
        }
        Err(error) => Err(CreateDirsAndWritePrettyJsonSyncErrorNamed::SerdeJson {
            error,
            code_occurence: error_occurence_lib::code_occurence!(),
        }),
    }
}
