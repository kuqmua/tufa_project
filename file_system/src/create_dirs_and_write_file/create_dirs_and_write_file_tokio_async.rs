use std::io::Error as IoError;
use std::path::Path;
use std::fs;
use tokio::io::AsyncWriteExt;
use error_occurence_lib::code_occurence::CodeOccurence;
use tokio::fs::File;

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum CreateDirsAndWriteFileTokioAsyncErrorNamed {
    StdIoError {
        #[eo_to_std_string_string]
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
            code_occurence: error_occurence_lib::code_occurence!(),
        });
    }
    match File::open(path).await {
        Ok(mut file) => {
            if let Err(error) = AsyncWriteExt::write_all(&mut file, bytes).await {
                return Err(CreateDirsAndWriteFileTokioAsyncErrorNamed::StdIoError {
                    error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
            Ok(())
        }
        Err(error) => Err(CreateDirsAndWriteFileTokioAsyncErrorNamed::StdIoError {
            error,
            code_occurence: error_occurence_lib::code_occurence!(),
        }),
    }
}
