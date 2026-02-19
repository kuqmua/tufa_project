use location_lib::{Location, loc, loc::Loc};
use serde_json::{Error as SerdeJsonEr, Value as SerdeJsonValue, to_string_pretty};
use std::path::Path;
use thiserror::Error;
#[derive(Debug, Error, Location)]
pub enum CreateDirsAndWritePrettyJsonSyncEr {
    SerdeJson {
        #[eo_to_err_string]
        er: SerdeJsonEr,
        loc: Loc,
    },
    WriteBytesIntoFile {
        #[eo_location]
        er: crate::CreateDirsAndWriteFileSyncEr,
        loc: Loc,
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
                    loc: loc!(),
                });
            }
            Ok(())
        }
        Err(er) => Err(CreateDirsAndWritePrettyJsonSyncEr::SerdeJson { er, loc: loc!() }),
    }
}
