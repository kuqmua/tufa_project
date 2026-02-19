use er_occurence_lib::{ErOccurence, code_occurence, code_occurence::CodeOccurence};
use serde_json::{Error as SerdeJsonEr, Value as SerdeJsonValue, to_string_pretty};
use std::path::Path;
use thiserror::Error;
#[derive(Debug, Error, ErOccurence)]
pub enum CreateDirsAndWritePrettyJsonSyncEr {
    SerdeJson {
        #[eo_to_err_string]
        er: SerdeJsonEr,
        code_occurence: CodeOccurence,
    },
    WriteBytesIntoFile {
        #[eo_er_occurence]
        er: crate::CreateDirsAndWriteFileSyncEr,
        code_occurence: CodeOccurence,
    },
}
pub fn create_dirs_and_write_pretty_json_sync(
    path: &Path,
    serde_json_value: &SerdeJsonValue,
) -> Result<(), CreateDirsAndWritePrettyJsonSyncEr> {
    match to_string_pretty(&serde_json_value) {
        Ok(value) => {
            if let Err(er) = crate::create_dirs_and_write_file_sync(path, value.as_bytes()) {
                return Err(CreateDirsAndWritePrettyJsonSyncEr::WriteBytesIntoFile {
                    er,
                    code_occurence: code_occurence!(),
                });
            }
            Ok(())
        }
        Err(er) => Err(CreateDirsAndWritePrettyJsonSyncEr::SerdeJson {
            er,
            code_occurence: code_occurence!(),
        }),
    }
}
