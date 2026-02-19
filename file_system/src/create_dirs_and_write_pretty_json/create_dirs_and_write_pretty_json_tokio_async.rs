use location_lib::{Location, code_occurence, code_occurence::CodeOccurence};
use serde_json::{Error as SerdeJsonEr, Value as SerdeJsonValue, to_string_pretty};
use std::path::Path;
use thiserror::Error;
#[derive(Debug, Error, Location)]
pub enum CreateDirsAndWritePrettyJsonTokioAsyncEr {
    SerdeJson {
        #[eo_to_err_string]
        er: SerdeJsonEr,
        code_occurence: CodeOccurence,
    },
    WriteBytesIntoFile {
        #[eo_location]
        er: crate::CreateDirsAndWriteFileTokioAsyncEr,
        code_occurence: CodeOccurence,
    },
}
pub async fn create_dirs_and_write_pretty_json_tokio_async(
    path: &Path,
    serde_json_value: SerdeJsonValue,
) -> Result<(), CreateDirsAndWritePrettyJsonTokioAsyncEr> {
    match to_string_pretty(&serde_json_value) {
        Ok(value) => {
            match crate::create_dirs_and_write_file_tokio_async(path, value.as_bytes()).await {
                Err(er) => Err(
                    CreateDirsAndWritePrettyJsonTokioAsyncEr::WriteBytesIntoFile {
                        er,
                        code_occurence: code_occurence!(),
                    },
                ),
                Ok(()) => Ok(()),
            }
        }
        Err(er) => Err(CreateDirsAndWritePrettyJsonTokioAsyncEr::SerdeJson {
            er,
            code_occurence: code_occurence!(),
        }),
    }
}
