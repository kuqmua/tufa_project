use crate::write_string_into_file::{path_with_rs_extension, try_write_string_into_path};
use optml::Optml;
use proc_macro2::TokenStream as Ts2;
use serde::Deserialize;
use std::{io, process::Command};
#[derive(Debug, Clone, Copy, Optml)]
pub enum FormatWithCargofmt {
    False,
    True,
}
#[derive(Debug, Copy, Clone, Deserialize, Optml)]
pub enum ShouldWriteTsIntoFile {
    False,
    True,
}
pub fn try_mb_write_ts_into_file(
    should_write_ts_into_file: ShouldWriteTsIntoFile,
    file_name: &str,
    ts: &Ts2,
    format_with_cargofmt: &FormatWithCargofmt,
) -> io::Result<()> {
    if matches!(should_write_ts_into_file, ShouldWriteTsIntoFile::False) {
        return Ok(());
    }
    let path = path_with_rs_extension(file_name);
    let ts_string = ts.to_string();
    try_write_string_into_path(&path, &ts_string)?;
    if matches!(format_with_cargofmt, FormatWithCargofmt::True) {
        let status = Command::new("rustfmt").arg(&path).status()?;
        if !status.success() {
            return Err(io::Error::other(format!(
                "rustfmt failed for {}",
                path.display()
            )));
        }
    }
    Ok(())
}
pub fn mb_write_ts_into_file(
    should_write_ts_into_file: ShouldWriteTsIntoFile,
    file_name: &str,
    ts: &Ts2,
    format_with_cargofmt: &FormatWithCargofmt,
) {
    if let Err(er) = try_mb_write_ts_into_file(
        should_write_ts_into_file,
        file_name,
        ts,
        format_with_cargofmt,
    ) {
        panic!("5ecc3880:{er}");
    }
}
#[cfg(test)]
mod tests {
    use super::{
        FormatWithCargofmt, ShouldWriteTsIntoFile, mb_write_ts_into_file, try_mb_write_ts_into_file,
    };
    use crate::{
        test_hlp::{cleanup_test_file, test_path},
        write_string_into_file::path_with_rs_extension,
    };
    use proc_macro2::TokenStream as Ts2;
    use std::fs::{metadata, read_to_string};
    #[test]
    fn mb_write_ts_into_file_skips_when_flag_is_false() {
        let base = test_path("macros_helpers_skip");
        let base_str = base.to_string_lossy().into_owned();
        let path = path_with_rs_extension(&base_str);
        let ts: Ts2 = "struct SkipWrite;".parse().expect("5994e7e2");
        mb_write_ts_into_file(
            ShouldWriteTsIntoFile::False,
            &base_str,
            &ts,
            &FormatWithCargofmt::False,
        );
        let _er = metadata(&path).expect_err("7be5f201");
    }
    #[test]
    fn mb_write_ts_into_file_writes_tokens_when_flag_is_true() {
        let base = test_path("macros_helpers_write");
        let base_str = base.to_string_lossy().into_owned();
        let path = path_with_rs_extension(&base_str);
        let ts: Ts2 = "struct DidWrite ;".parse().expect("6c20f49a");
        let expected = ts.to_string();
        mb_write_ts_into_file(
            ShouldWriteTsIntoFile::True,
            &base_str,
            &ts,
            &FormatWithCargofmt::False,
        );
        let actual = read_to_string(&path).expect("dcfd3d1d");
        assert_eq!(actual, expected);
        cleanup_test_file(path);
    }
    #[test]
    fn try_mb_write_ts_into_file_writes_tokens_when_enabled() {
        let base = test_path("macros_helpers_try_write");
        let base_str = base.to_string_lossy().into_owned();
        let path = path_with_rs_extension(&base_str);
        let ts: Ts2 = "struct TryDidWrite ;".parse().expect("f771ac2d");
        let expected = ts.to_string();
        try_mb_write_ts_into_file(
            ShouldWriteTsIntoFile::True,
            &base_str,
            &ts,
            &FormatWithCargofmt::False,
        )
        .expect("6fee9f6f");
        let actual = read_to_string(&path).expect("c53ea835");
        assert_eq!(actual, expected);
        cleanup_test_file(path);
    }
}
