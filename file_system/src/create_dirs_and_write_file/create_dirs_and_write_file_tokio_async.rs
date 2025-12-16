#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum CreateDirsAndWriteFileTokioAsyncErrorNamed {
    StdIoError {
        #[eo_to_std_string_string]
        error: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub async fn create_dirs_and_write_file_tokio_async(path: &std::path::Path, bytes: &[u8]) -> Result<(), CreateDirsAndWriteFileTokioAsyncErrorNamed> {
    if let Some(prefix) = path.parent()
        && let Err(error) = std::fs::create_dir_all(prefix)
    {
        return Err(CreateDirsAndWriteFileTokioAsyncErrorNamed::StdIoError { error, code_occurence: error_occurence_lib::code_occurence!() });
    }
    match tokio::fs::File::open(path).await {
        Ok(mut file) => {
            if let Err(error) = tokio::io::AsyncWriteExt::write_all(&mut file, bytes).await {
                return Err(CreateDirsAndWriteFileTokioAsyncErrorNamed::StdIoError { error, code_occurence: error_occurence_lib::code_occurence!() });
            }
            Ok(())
        }
        Err(error) => Err(CreateDirsAndWriteFileTokioAsyncErrorNamed::StdIoError { error, code_occurence: error_occurence_lib::code_occurence!() }),
    }
}
