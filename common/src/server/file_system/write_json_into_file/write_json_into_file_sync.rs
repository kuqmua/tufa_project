#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum WriteJsonIntoFileSyncErrorNamed {
    SerdeJson{
        #[eo_to_std_string_string]
        serde_json: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WriteBytesIntoFile {
        #[eo_error_occurence]
        write_bytes_into_file: crate::server::file_system::write_bytes_into_file::write_bytes_into_file_sync::WriteBytesIntoFileSyncErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub fn write_json_into_file_async(
    path: &std::path::Path,
    json_object: serde_json::Value,
) -> Result<(), Box<WriteJsonIntoFileSyncErrorNamed>> {
    match serde_json::to_string_pretty(&json_object) {
        Err(error) => Err(Box::new(WriteJsonIntoFileSyncErrorNamed::SerdeJson {
            serde_json: error,
            code_occurence: error_occurence_lib::code_occurence!(),
        })),
        Ok(stringified_json) => {
            if let Err(error) = crate::server::file_system::write_bytes_into_file::write_bytes_into_file_sync::write_bytes_into_file_sync(
                path,
                stringified_json,
            ) {
                return Err(
                    Box::new(
                        WriteJsonIntoFileSyncErrorNamed::WriteBytesIntoFile{
                            write_bytes_into_file: *e,
                            code_occurence: error_occurence_lib::code_occurence!()
                        }
                    )
                );
            }
            Ok(())
        }
    }
}
