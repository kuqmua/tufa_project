#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum WriteJsonIntoFileSyncErrorNamed {
    SerdeJson{
        #[eo_to_std_string_string]
        error: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WriteBytesIntoFile {
        #[eo_error_occurence]
        error: crate::file_system::write_bytes_into_file::write_bytes_into_file_sync::WriteBytesIntoFileSyncErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub fn write_json_into_file_async(path: &std::path::Path, json_object: serde_json::Value) -> Result<(), WriteJsonIntoFileSyncErrorNamed> {
    match serde_json::to_string_pretty(&json_object) {
        Ok(stringified_json) => {
            if let Err(error) = crate::file_system::write_bytes_into_file::write_bytes_into_file_sync::write_bytes_into_file_sync(
                path,
                stringified_json.as_bytes(),
            ) {
                return Err(WriteJsonIntoFileSyncErrorNamed::WriteBytesIntoFile{
                    error,
                    code_occurence: error_occurence_lib::code_occurence!()
                });
            }
            Ok(())
        },
        Err(error) => Err(WriteJsonIntoFileSyncErrorNamed::SerdeJson {
            error,
            code_occurence: error_occurence_lib::code_occurence!(),
        })
    }
}
