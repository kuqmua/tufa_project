use std::{
    fs::write,
    io,
    path::{Path, PathBuf},
};
pub(crate) fn try_write_string_into_path(path: &Path, string_cnt: &str) -> io::Result<()> {
    write(path, string_cnt)
}
pub(crate) fn path_with_rs_extension(file_name: &str) -> PathBuf {
    PathBuf::from(file_name).with_extension("rs")
}
pub fn try_write_string_into_file(file_name: &str, string_cnt: &str) -> io::Result<PathBuf> {
    let path = path_with_rs_extension(file_name);
    try_write_string_into_path(&path, string_cnt)?;
    Ok(path)
}
pub fn write_string_into_file(file_name: &str, string_cnt: &str) {
    if let Err(er) = try_write_string_into_file(file_name, string_cnt) {
        panic!("4f3094e1:{er}");
    }
}
#[cfg(test)]
mod tests {
    use super::{
        path_with_rs_extension, try_write_string_into_file, try_write_string_into_path,
        write_string_into_file,
    };
    use crate::test_hlp::{cleanup_test_file, test_path};
    use std::fs::read_to_string;
    #[test]
    fn write_string_into_path_writes_exact_content() {
        let path = test_path("macros_helpers_write_path").with_extension("txt");
        try_write_string_into_path(&path, "abc").expect("dcb22948");
        let ts = read_to_string(&path).expect("90d9af53");
        assert_eq!(ts, "abc");
        cleanup_test_file(path);
    }
    #[test]
    fn write_string_into_file_adds_rs_extension() {
        let base = test_path("macros_helpers_write_file");
        let base_str = base.to_string_lossy().into_owned();
        let path = path_with_rs_extension(&base_str);
        write_string_into_file(&base_str, "xyz");
        let ts = read_to_string(&path).expect("52d80eb4");
        assert_eq!(ts, "xyz");
        cleanup_test_file(path);
    }
    #[test]
    fn try_write_string_into_file_returns_path() {
        let base = test_path("macros_helpers_try_write_file");
        let base_str = base.to_string_lossy().into_owned();
        let path = try_write_string_into_file(&base_str, "qwe").expect("6676e082");
        let ts = read_to_string(&path).expect("77411ea0");
        assert_eq!(ts, "qwe");
        cleanup_test_file(path);
    }
}
