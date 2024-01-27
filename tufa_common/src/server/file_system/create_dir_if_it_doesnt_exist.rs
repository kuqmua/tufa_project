#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum CreateDirIfItDoesntExistErrorNamed {
    CreateDirAll {
        #[eo_display]
        error: std::io::Error,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

pub fn create_dir_if_it_doesnt_exist(
    path: &str,
) -> Result<(), Box<CreateDirIfItDoesntExistErrorNamed>> {
    if std::path::Path::new(path).exists() {
        return Ok(());
    }
    if let Err(e) = std::fs::create_dir_all(path) {
        return Err(Box::new(CreateDirIfItDoesntExistErrorNamed::CreateDirAll {
            error: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        }));
    }
    Ok(())
}
