use location_lib::{Location, loc, loc::Loc};
use optml::Optml;
use serde_json::{Error as SerdeJsonEr, Value as SerdeJsonV, to_string_pretty};
use std::path::Path;
use thiserror::Error;
#[derive(Debug, Error, Location, Optml)]
pub enum CrDirsAndWritePrettyJsonTokioAsyncEr {
    SerdeJson {
        #[eo_to_err_string]
        er: SerdeJsonEr,
        loc: Loc,
    },
    WriteBytesIntoFile {
        #[eo_location]
        er: crate::CrDirsAndWriteFileTokioAsyncEr,
        loc: Loc,
    },
}
pub async fn cr_dirs_and_write_pretty_json_tokio_async(
    path: &Path,
    serde_json_v: SerdeJsonV,
) -> Result<(), CrDirsAndWritePrettyJsonTokioAsyncEr> {
    match to_string_pretty(&serde_json_v) {
        Ok(v) => match crate::cr_dirs_and_write_file_tokio_async(path, v.as_bytes()).await {
            Err(er) => {
                Err(CrDirsAndWritePrettyJsonTokioAsyncEr::WriteBytesIntoFile { er, loc: loc!() })
            }
            Ok(()) => Ok(()),
        },
        Err(er) => Err(CrDirsAndWritePrettyJsonTokioAsyncEr::SerdeJson { er, loc: loc!() }),
    }
}
