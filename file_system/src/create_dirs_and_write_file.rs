mod create_dirs_and_write_file_sync;
mod create_dirs_and_write_file_tokio_async;
pub use create_dirs_and_write_file_sync::{
    CreateDirsAndWriteFileSyncEr, CreateDirsAndWriteFileSyncErWithSerde,
    create_dirs_and_write_file_sync,
};
pub use create_dirs_and_write_file_tokio_async::{
    CreateDirsAndWriteFileTokioAsyncEr, CreateDirsAndWriteFileTokioAsyncErWithSerde,
    create_dirs_and_write_file_tokio_async,
};
