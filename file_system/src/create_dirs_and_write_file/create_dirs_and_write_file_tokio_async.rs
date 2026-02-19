use location_lib::{Location, loc, loc::Loc};
use std::{fs, io::Error as IoEr, path::Path};
use thiserror::Error;
use tokio::{fs::File, io::AsyncWriteExt};
#[derive(Debug, Error, Location)]
pub enum CreateDirsAndWriteFileTokioAsyncEr {
    StdIoEr {
        #[eo_to_err_string]
        er: IoEr,
        loc: Loc,
    },
}
pub async fn create_dirs_and_write_file_tokio_async(
    path: &Path,
    bytes: &[u8],
) -> Result<(), CreateDirsAndWriteFileTokioAsyncEr> {
    if let Some(prefix) = path.parent()
        && let Err(er) = fs::create_dir_all(prefix)
    {
        return Err(CreateDirsAndWriteFileTokioAsyncEr::StdIoEr { er, loc: loc!() });
    }
    match File::open(path).await {
        Ok(mut file) => {
            if let Err(er) = AsyncWriteExt::write_all(&mut file, bytes).await {
                return Err(CreateDirsAndWriteFileTokioAsyncEr::StdIoEr { er, loc: loc!() });
            }
            Ok(())
        }
        Err(er) => Err(CreateDirsAndWriteFileTokioAsyncEr::StdIoEr { er, loc: loc!() }),
    }
}
