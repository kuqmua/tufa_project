use location_lib::{Location, loc, loc::Loc};
use std::{fs, io::Error as IoEr, path::Path};
use thiserror::Error;
#[derive(Debug, Error, Location)]
pub enum CreateDirIfItDoesntExistEr {
    CreateDirAll {
        #[eo_to_err_string]
        er: IoEr,
        loc: Loc,
    },
}
pub fn create_dir_if_it_doesnt_exist(path: &str) -> Result<(), CreateDirIfItDoesntExistEr> {
    if Path::new(path).exists() {
        return Ok(());
    }
    if let Err(er) = fs::create_dir_all(path) {
        return Err(CreateDirIfItDoesntExistEr::CreateDirAll { er, loc: loc!() });
    }
    Ok(())
}
