#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum WriteBytesIntoFileSyncErrorNamed {
    StdIo {
        #[eo_display]
        std_io_error: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub fn write_bytes_into_file_sync(
    path: &std::path::Path,
    bytes: std::string::String,
) -> Result<(), Box<WriteBytesIntoFileSyncErrorNamed>> {
    if let Some(prefix) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(prefix) {
            return Err(Box::new(WriteBytesIntoFileSyncErrorNamed::StdIo {
                std_io_error: e,
                code_occurence: error_occurence_lib::code_occurence!(),
            }));
        }
    }
    match std::fs::File::create(path) {
        Err(e) => Err(Box::new(WriteBytesIntoFileSyncErrorNamed::StdIo {
            std_io_error: e,
            code_occurence: error_occurence_lib::code_occurence!(),
        })),
        Ok(mut file) => {
            if let Err(e) = std::io::Write::write_all(&mut file, bytes.as_bytes()) {
                return Err(Box::new(WriteBytesIntoFileSyncErrorNamed::StdIo {
                    std_io_error: e,
                    code_occurence: error_occurence_lib::code_occurence!(),
                }));
            }
            if let Err(e) = file.sync_all() {
                return Err(Box::new(WriteBytesIntoFileSyncErrorNamed::StdIo {
                    std_io_error: e,
                    code_occurence: error_occurence_lib::code_occurence!(),
                }));
            }
            Ok(())
        }
    }
}
