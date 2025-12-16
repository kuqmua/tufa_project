#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum WriteBytesIntoFileAsyncTokioErrorNamed {
    StdIoError {
        #[eo_to_std_string_string]
        error: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub async fn write_bytes_into_file_async_tokio(path: &std::path::Path, bytes: &[u8]) -> Result<(), WriteBytesIntoFileAsyncTokioErrorNamed> {
    if let Some(prefix) = path.parent() && let Err(error) = std::fs::create_dir_all(prefix) {
        return Err(WriteBytesIntoFileAsyncTokioErrorNamed::StdIoError {
            error,
            code_occurence: error_occurence_lib::code_occurence!(),
        });
    }
    match tokio::fs::File::open(path).await {
        Ok(mut file) => {
            if let Err(error) = tokio::io::AsyncWriteExt::write_all(&mut file, bytes).await {
                return Err(WriteBytesIntoFileAsyncTokioErrorNamed::StdIoError {
                    error,
                    code_occurence: error_occurence_lib::code_occurence!(),
                });
            }
            Ok(())
        },
        Err(error) => Err(WriteBytesIntoFileAsyncTokioErrorNamed::StdIoError {
            error,
            code_occurence: error_occurence_lib::code_occurence!(),
        })
    }
}
