use error_occurence_lib::{ErrorOccurence, code_occurence, code_occurence::CodeOccurence};
use std::{
    fs::{self, File},
    io::{Error as IoError, Write},
    path::Path,
};
use thiserror::Error;
#[derive(Debug, Error, ErrorOccurence)]
pub enum CreateDirsAndWriteFileSyncError {
    StdIo {
        #[eo_to_err_string]
        er: IoError,
        code_occurence: CodeOccurence,
    },
}
pub fn create_dirs_and_write_file_sync(
    path: &Path,
    bytes: &[u8],
) -> Result<(), CreateDirsAndWriteFileSyncError> {
    if let Some(prefix) = path.parent()
        && let Err(er) = fs::create_dir_all(prefix)
    {
        return Err(CreateDirsAndWriteFileSyncError::StdIo {
            er,
            code_occurence: code_occurence!(),
        });
    }
    match File::create(path) {
        Ok(mut file) => {
            if let Err(er) = Write::write_all(&mut file, bytes) {
                return Err(CreateDirsAndWriteFileSyncError::StdIo {
                    er,
                    code_occurence: code_occurence!(),
                });
            }
            if let Err(er) = file.sync_all() {
                return Err(CreateDirsAndWriteFileSyncError::StdIo {
                    er,
                    code_occurence: code_occurence!(),
                });
            }
            Ok(())
        }
        Err(er) => Err(CreateDirsAndWriteFileSyncError::StdIo {
            er,
            code_occurence: code_occurence!(),
        }),
    }
}
