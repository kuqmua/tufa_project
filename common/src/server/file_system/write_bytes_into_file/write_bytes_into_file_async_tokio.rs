#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum WriteBytesIntoFileAsyncTokioErrorNamed {
    StdIoError {
        #[eo_display]
        std_io_error: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub async fn write_bytes_into_file_async_tokio<'a>(
    path: &'a std::path::Path,
    bytes: &[u8],
) -> Result<(), Box<WriteBytesIntoFileAsyncTokioErrorNamed>> {
    if let Some(prefix) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(prefix) {
            return Err(Box::new(
                WriteBytesIntoFileAsyncTokioErrorNamed::StdIoError {
                    std_io_error: e,
                    code_occurence: error_occurence_lib::code_occurence!(),
                },
            ));
        }
    }
    match tokio::fs::File::open(path).await {
        Err(e) => Err(Box::new(
            WriteBytesIntoFileAsyncTokioErrorNamed::StdIoError {
                std_io_error: e,
                code_occurence: error_occurence_lib::code_occurence!(),
            },
        )),
        Ok(mut file) => {
            if let Err(e) = tokio::io::AsyncWriteExt::write_all(&mut file, bytes).await {
                return Err(Box::new(
                    WriteBytesIntoFileAsyncTokioErrorNamed::StdIoError {
                        std_io_error: e,
                        code_occurence: error_occurence_lib::code_occurence!(),
                    },
                ));
            }
            Ok(())
        }
    }
}
