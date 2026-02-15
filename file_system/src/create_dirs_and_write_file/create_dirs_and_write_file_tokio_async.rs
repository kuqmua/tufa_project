use error_occurence_lib::{ErrorOccurence, code_occurence, code_occurence::CodeOccurence};
use std::{fs, io::Error as IoError, path::Path};
use thiserror::Error;
use tokio::{fs::File, io::AsyncWriteExt};
#[derive(Debug, Error, ErrorOccurence)]
pub enum CreateDirsAndWriteFileTokioAsyncErrorNamed {
    StdIoError {
        #[eo_to_err_string]
        error: IoError,
        code_occurence: CodeOccurence,
    },
}
pub async fn create_dirs_and_write_file_tokio_async(
    path: &Path,
    bytes: &[u8],
) -> Result<(), CreateDirsAndWriteFileTokioAsyncErrorNamed> {
    if let Some(prefix) = path.parent()
        && let Err(error) = fs::create_dir_all(prefix)
    {
        return Err(CreateDirsAndWriteFileTokioAsyncErrorNamed::StdIoError {
            error,
            code_occurence: code_occurence!(),
        });
    }
    match File::open(path).await {
        Ok(mut file) => {
            if let Err(error) = AsyncWriteExt::write_all(&mut file, bytes).await {
                return Err(CreateDirsAndWriteFileTokioAsyncErrorNamed::StdIoError {
                    error,
                    code_occurence: code_occurence!(),
                });
            }
            Ok(())
        }
        Err(error) => Err(CreateDirsAndWriteFileTokioAsyncErrorNamed::StdIoError {
            error,
            code_occurence: code_occurence!(),
        }),
    }
}
