use location_lib::{Location, loc, loc::Loc};
use optml::Optml;
use serde_json::{Error as SerdeJsonEr, Value as SerdeJsonV, to_string_pretty};
use std::path::Path;
use thiserror::Error;
#[derive(Debug, Error, Location, Optml)]
pub enum CrDirsAndWritePrettyJsonSyncEr {
    SerdeJson {
        #[eo_to_err_string]
        er: SerdeJsonEr,
        loc: Loc,
    },
    WriteBytesIntoFile {
        #[eo_location]
        er: crate::CrDirsAndWriteFileSyncEr,
        loc: Loc,
    },
}
pub fn cr_dirs_and_write_pretty_json_sync(
    path: &Path,
    serde_json_v: &SerdeJsonV,
) -> Result<(), CrDirsAndWritePrettyJsonSyncEr> {
    match to_string_pretty(&serde_json_v) {
        Ok(v) => {
            if let Err(er) = crate::cr_dirs_and_write_file_sync(path, v.as_bytes()) {
                return Err(CrDirsAndWritePrettyJsonSyncEr::WriteBytesIntoFile { er, loc: loc!() });
            }
            Ok(())
        }
        Err(er) => Err(CrDirsAndWritePrettyJsonSyncEr::SerdeJson { er, loc: loc!() }),
    }
}
