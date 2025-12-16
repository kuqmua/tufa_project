#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum WritePrettyJsonIntoFileAsyncTokioErrorNamed {
    SerdeJson {
        #[eo_to_std_string_string]
        error: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WriteBytesIntoFile {
        #[eo_error_occurence] 
        error: crate::file_system::write_bytes_into_file::create_dir_all_tokio_fs_file_open_tokio_io_async_write_ext_write_all::CreateDirAllTokioFsFileOpenTokioIoAsyncWriteExtWriteAllErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    }, 
}

pub async fn write_pretty_json_into_file_async_tokio(path: &std::path::Path, serde_json_value: serde_json::Value) -> Result<(), WritePrettyJsonIntoFileAsyncTokioErrorNamed> {
    match serde_json::to_string_pretty(&serde_json_value) {
        Ok(value) => match crate::file_system::write_bytes_into_file::create_dir_all_tokio_fs_file_open_tokio_io_async_write_ext_write_all::create_dir_all_tokio_fs_file_open_tokio_io_async_write_ext_write_all(
            path,
            value.as_bytes(),
        )
        .await {
            Err(error) => Err(WritePrettyJsonIntoFileAsyncTokioErrorNamed::WriteBytesIntoFile {
                error,
                code_occurence: error_occurence_lib::code_occurence!() 
            }),
            Ok(()) => Ok(())
        },
        Err(error) => Err(WritePrettyJsonIntoFileAsyncTokioErrorNamed::SerdeJson {
            error,
            code_occurence: error_occurence_lib::code_occurence!() 
        })
    }
}