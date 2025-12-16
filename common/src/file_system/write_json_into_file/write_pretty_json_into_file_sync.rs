#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum WritePrettyJsonIntoFileSyncErrorNamed {
    SerdeJson{
        #[eo_to_std_string_string]
        error: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WriteBytesIntoFile {
        #[eo_error_occurence]
        error: crate::file_system::write_bytes_into_file::create_dir_all_std_fs_file_create_std_io_write_write_all_sync_all::CreateDirAllStdFsFileCreateStdIoWriteWriteAllSyncAllErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub fn write_pretty_json_into_file_sync(path: &std::path::Path, serde_json_value: serde_json::Value) -> Result<(), WritePrettyJsonIntoFileSyncErrorNamed> {
    match serde_json::to_string_pretty(&serde_json_value) {
        Ok(value) => {
            if let Err(error) = crate::file_system::write_bytes_into_file::create_dir_all_std_fs_file_create_std_io_write_write_all_sync_all::create_dir_all_std_fs_file_create_std_io_write_write_all_sync_all(
                path,
                value.as_bytes(),
            ) {
                return Err(WritePrettyJsonIntoFileSyncErrorNamed::WriteBytesIntoFile{
                    error,
                    code_occurence: error_occurence_lib::code_occurence!()
                });
            }
            Ok(())
        },
        Err(error) => Err(WritePrettyJsonIntoFileSyncErrorNamed::SerdeJson {
            error,
            code_occurence: error_occurence_lib::code_occurence!(),
        })
    }
}
