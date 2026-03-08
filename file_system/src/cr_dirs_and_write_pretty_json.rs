mod cr_dirs_and_write_pretty_json_sync;
mod cr_dirs_and_write_pretty_json_tokio_async;
pub use cr_dirs_and_write_pretty_json_sync::{
    CrDirsAndWritePrettyJsonSyncEr, CrDirsAndWritePrettyJsonSyncErWithSerde,
    cr_dirs_and_write_pretty_json_sync,
};
pub use cr_dirs_and_write_pretty_json_tokio_async::{
    CrDirsAndWritePrettyJsonTokioAsyncEr, CrDirsAndWritePrettyJsonTokioAsyncErWithSerde,
    cr_dirs_and_write_pretty_json_tokio_async,
};
