use location_lib::{Location, loc, loc::Loc};
use optimal_pack::OptimalPack;
use serde_json::{Error as SerdeJsonEr, Value as SerdeJsonV, to_string_pretty};
use std::path::Path;
use thiserror::Error;
#[derive(Debug, Error, Location, OptimalPack)]
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
    serde_json_v: &SerdeJsonV,
) -> Result<(), CreateDirsAndWritePrettyJsonSyncEr> {
    match to_string_pretty(&serde_json_v) {
        Ok(v) => {
            if let Err(er) = crate::create_dirs_and_write_file_sync(path, v.as_bytes()) {
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
