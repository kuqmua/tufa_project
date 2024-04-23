#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum CreateDirIfItDoesntExistErrorNamed {
    CreateDirAll {
        #[eo_display]
        error: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

pub fn create_dir_if_it_doesnt_exist(
    path: &str,
) -> Result<(), Box<CreateDirIfItDoesntExistErrorNamed>> {
    if std::path::Path::new(path).exists() {
        return Ok(());
    }
    if let Err(error) = std::fs::create_dir_all(path) {
        return Err(Box::new(CreateDirIfItDoesntExistErrorNamed::CreateDirAll {
            error: error,
            code_occurence: error_occurence_lib::code_occurence!(),
        }));
    }
    Ok(())
}
