use error_occurence_lib::{ErrorOccurence, code_occurence, code_occurence::CodeOccurence};
use std::{fs, io::Error as IoError, path::Path};
use thiserror::Error;
#[derive(Debug, Error, ErrorOccurence)]
pub enum CreateDirIfItDoesntExistErrorNamed {
    CreateDirAll {
        #[eo_to_err_string]
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
            code_occurence: code_occurence!(),
        });
    }
    Ok(())
}
