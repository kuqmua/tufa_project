use location_lib::{Location, code_occurence, code_occurence::CodeOccurence};
use std::{
    fs::{self, File},
    io::{Error as IoEr, Write},
    path::Path,
};
use thiserror::Error;
#[derive(Debug, Error, Location)]
pub enum CreateDirsAndWriteFileSyncEr {
    StdIo {
        #[eo_to_err_string]
        er: IoEr,
        code_occurence: CodeOccurence,
    },
}
pub fn create_dirs_and_write_file_sync(
    path: &Path,
    bytes: &[u8],
) -> Result<(), CreateDirsAndWriteFileSyncEr> {
    if let Some(prefix) = path.parent()
        && let Err(er) = fs::create_dir_all(prefix)
    {
        return Err(CreateDirsAndWriteFileSyncEr::StdIo {
            er,
            code_occurence: code_occurence!(),
        });
    }
    match File::create(path) {
        Ok(mut file) => {
            if let Err(er) = Write::write_all(&mut file, bytes) {
                return Err(CreateDirsAndWriteFileSyncEr::StdIo {
                    er,
                    code_occurence: code_occurence!(),
                });
            }
            if let Err(er) = file.sync_all() {
                return Err(CreateDirsAndWriteFileSyncEr::StdIo {
                    er,
                    code_occurence: code_occurence!(),
                });
            }
            Ok(())
        }
        Err(er) => Err(CreateDirsAndWriteFileSyncEr::StdIo {
            er,
            code_occurence: code_occurence!(),
        }),
    }
}
