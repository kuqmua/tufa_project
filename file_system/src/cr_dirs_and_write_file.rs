mod cr_dirs_and_write_file_sync;
mod cr_dirs_and_write_file_tokio_async;
pub use cr_dirs_and_write_file_sync::{
    CrDirsAndWriteFileSyncEr, CrDirsAndWriteFileSyncErWithSerde, cr_dirs_and_write_file_sync,
};
pub use cr_dirs_and_write_file_tokio_async::{
    CrDirsAndWriteFileTokioAsyncEr, CrDirsAndWriteFileTokioAsyncErWithSerde,
    cr_dirs_and_write_file_tokio_async,
};
