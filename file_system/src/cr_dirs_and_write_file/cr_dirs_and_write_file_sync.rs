use location_lib::{Location, loc, loc::Loc};
use optml::Optml;
use std::{
    fs::{self, File},
    io::{Error as IoEr, Write},
    path::Path,
};
use thiserror::Error;
#[derive(Debug, Error, Location, Optml)]
pub enum CrDirsAndWriteFileSyncEr {
    StdIo {
        #[eo_to_err_string]
        er: IoEr,
        loc: Loc,
    },
}
pub fn cr_dirs_and_write_file_sync(
    path: &Path,
    bytes: &[u8],
) -> Result<(), CrDirsAndWriteFileSyncEr> {
    if let Some(prefix) = path.parent()
        && let Err(er) = fs::create_dir_all(prefix)
    {
        return Err(CrDirsAndWriteFileSyncEr::StdIo { er, loc: loc!() });
    }
    match File::create(path) {
        Ok(mut file) => {
            if let Err(er) = Write::write_all(&mut file, bytes) {
                return Err(CrDirsAndWriteFileSyncEr::StdIo { er, loc: loc!() });
            }
            if let Err(er) = file.sync_all() {
                return Err(CrDirsAndWriteFileSyncEr::StdIo { er, loc: loc!() });
            }
            Ok(())
        }
        Err(er) => Err(CrDirsAndWriteFileSyncEr::StdIo { er, loc: loc!() }),
    }
}
