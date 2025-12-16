#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum CreateDirsAndWritePrettyJsonSyncErrorNamed {
    SerdeJson{
        #[eo_to_std_string_string]
        error: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WriteBytesIntoFile {
        #[eo_error_occurence]
        error: crate::write_bytes_into_file::create_dirs_and_write_file_sync::CreateDirsAndWriteFileSyncErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub fn create_dirs_and_write_pretty_json_sync(path: &std::path::Path, serde_json_value: serde_json::Value) -> Result<(), CreateDirsAndWritePrettyJsonSyncErrorNamed> {
    match serde_json::to_string_pretty(&serde_json_value) {
        Ok(value) => {
            if let Err(error) = crate::write_bytes_into_file::create_dirs_and_write_file_sync::create_dirs_and_write_file_sync(
                path,
                value.as_bytes(),
            ) {
                return Err(CreateDirsAndWritePrettyJsonSyncErrorNamed::WriteBytesIntoFile{
                    error,
                    code_occurence: error_occurence_lib::code_occurence!()
                });
            }
            Ok(())
        },
        Err(error) => Err(CreateDirsAndWritePrettyJsonSyncErrorNamed::SerdeJson {
            error,
            code_occurence: error_occurence_lib::code_occurence!(),
        })
    }
}
