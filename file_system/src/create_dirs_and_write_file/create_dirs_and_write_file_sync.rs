#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum CreateDirsAndWriteFileSyncErrorNamed {
    StdIo {
        #[eo_to_std_string_string]
        error: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub fn create_dirs_and_write_file_sync(
    path: &std::path::Path,
    bytes: &[u8],
) -> Result<(), CreateDirsAndWriteFileSyncErrorNamed> {
    if let Some(prefix) = path.parent()
        && let Err(error) = std::fs::create_dir_all(prefix)
    {
        return Err(CreateDirsAndWriteFileSyncErrorNamed::StdIo {
            error,
            code_occurence: error_occurence_lib::code_occurence!(),
        });
    }
    match std::fs::File::create(path) {
        Ok(mut file) => {
            if let Err(error) = std::io::Write::write_all(&mut file, bytes) {
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
