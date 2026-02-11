use error_occurence_lib::code_occurence::CodeOccurence;
use std::{
    fs::{self, File},
    io::{Error as IoError, Write},
    path::Path,
};

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum CreateDirsAndWriteFileSyncErrorNamed {
    StdIo {
        #[eo_to_std_string_string]
        error: IoError,
        code_occurence: CodeOccurence,
    },
}

pub fn create_dirs_and_write_file_sync(
    path: &Path,
    bytes: &[u8],
) -> Result<(), CreateDirsAndWriteFileSyncErrorNamed> {
    if let Some(prefix) = path.parent()
        && let Err(error) = fs::create_dir_all(prefix)
    {
        return Err(CreateDirsAndWriteFileSyncErrorNamed::StdIo {
            error,
            code_occurence: error_occurence_lib::code_occurence!(),
        });
    }
    match File::create(path) {
        Ok(mut file) => {
            if let Err(error) = Write::write_all(&mut file, bytes) {
                return Err(CreateDirsAndWriteFileSyncErrorNamed::StdIo {
                    error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
            if let Err(error) = file.sync_all() {
                return Err(CreateDirsAndWriteFileSyncErrorNamed::StdIo {
                    error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
            Ok(())
        }
        Err(error) => Err(CreateDirsAndWriteFileSyncErrorNamed::StdIo {
            error,
            code_occurence: error_occurence_lib::code_occurence!(),
        }),
    }
}
