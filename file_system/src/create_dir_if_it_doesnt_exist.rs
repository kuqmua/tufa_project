use error_occurence_lib::code_occurence::CodeOccurence;
use std::{fs, io::Error as IoError, path::Path};

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum CreateDirIfItDoesntExistErrorNamed {
    CreateDirAll {
        #[eo_to_std_string_string]
        error: IoError,
        code_occurence: CodeOccurence,
    },
}

pub fn create_dir_if_it_doesnt_exist(path: &str) -> Result<(), CreateDirIfItDoesntExistErrorNamed> {
    if Path::new(path).exists() {
        return Ok(());
    }
    if let Err(error) = fs::create_dir_all(path) {
        return Err(CreateDirIfItDoesntExistErrorNamed::CreateDirAll {
            error,
            code_occurence: error_occurence_lib::code_occurence!(),
        });
    }
    Ok(())
}
