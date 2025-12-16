#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum CreateDirAllTokioFsFileOpenTokioIoAsyncWriteExtWriteAllErrorNamed {
    StdIoError {
        #[eo_to_std_string_string]
        error: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub async fn create_dir_all_tokio_fs_file_open_tokio_io_async_write_ext_write_all(path: &std::path::Path, bytes: &[u8]) -> Result<(), CreateDirAllTokioFsFileOpenTokioIoAsyncWriteExtWriteAllErrorNamed> {
    if let Some(prefix) = path.parent() && let Err(error) = std::fs::create_dir_all(prefix) {
        return Err(CreateDirAllTokioFsFileOpenTokioIoAsyncWriteExtWriteAllErrorNamed::StdIoError {
            error,
            code_occurence: error_occurence_lib::code_occurence!(),
        });
    }
    match tokio::fs::File::open(path).await {
        Ok(mut file) => {
            if let Err(error) = tokio::io::AsyncWriteExt::write_all(&mut file, bytes).await {
                return Err(CreateDirAllTokioFsFileOpenTokioIoAsyncWriteExtWriteAllErrorNamed::StdIoError {
                    error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
            Ok(())
        },
        Err(error) => Err(CreateDirAllTokioFsFileOpenTokioIoAsyncWriteExtWriteAllErrorNamed::StdIoError {
            error,
            code_occurence: error_occurence_lib::code_occurence!(),
        })
    }
}
