use location_lib::{Location, loc, loc::Loc};
use optml::Optml;
use std::{fs, io::Error as IoEr, path::Path};
use thiserror::Error;
#[derive(Debug, Error, Location, Optml)]
pub enum CrDirIfItDoesntExistEr {
    CrDirAll {
        #[eo_to_err_string]
        er: IoEr,
        loc: Loc,
    },
}
pub fn cr_dir_if_it_doesnt_exist(v: &str) -> Result<(), CrDirIfItDoesntExistEr> {
    if Path::new(v).exists() {
        return Ok(());
    }
    if let Err(er) = fs::create_dir_all(v) {
        return Err(CrDirIfItDoesntExistEr::CrDirAll { er, loc: loc!() });
    }
    Ok(())
}
