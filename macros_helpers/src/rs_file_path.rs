use std::path::{Path, PathBuf};
#[allow(clippy::single_call_fn)] // centralized .rs extension mapping keeps path behavior consistent across file-write helpers
pub(crate) fn rs_file_path<P>(file_name: P) -> PathBuf
where
    P: AsRef<Path>,
{
    file_name.as_ref().with_extension("rs")
}
#[cfg(test)]
mod tests {
    use super::rs_file_path;
    use std::path::Path;
    #[test]
    fn rs_file_path_adds_rs_extension_for_path_without_extension() {
        let actual = rs_file_path("src/generated");
        assert_eq!(actual, Path::new("src/generated.rs"));
    }
    #[test]
    fn rs_file_path_replaces_existing_extension() {
        let actual = rs_file_path("src/generated.txt");
        assert_eq!(actual, Path::new("src/generated.rs"));
    }
    #[test]
    fn rs_file_path_keeps_parent_directories() {
        let actual = rs_file_path("tmp/a/b/c");
        assert_eq!(actual, Path::new("tmp/a/b/c.rs"));
    }
}
