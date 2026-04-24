use crate::panic_if_err::panic_if_err;
use crate::write_string_into_file::{WritePathOutcome, try_write_string_into_file_with_outcome};
use optml::Optml;
use proc_macro2::TokenStream as Ts2;
use serde::Deserialize;
use std::{io, path::Path, process::Command};
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
#[allow(clippy::single_call_fn)] // rustfmt execution is isolated so io/process errors stay localized and easy to test
fn try_run_rustfmt(path: &Path) -> io::Result<()> {
    let status = Command::new("rustfmt").arg(path).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "rustfmt failed for {}",
            path.display()
        )))
    }
}
#[allow(clippy::single_call_fn)] // keeps ShouldWriteTsIntoFile flag interpretation centralized
const fn should_write_ts_flag(v: ShouldWriteTsIntoFile) -> bool {
    matches!(v, ShouldWriteTsIntoFile::True)
}
#[allow(clippy::single_call_fn)] // centralizes token-to-file write mapping and outcome extraction
fn try_write_ts_into_file<P>(file_name: P, ts: &Ts2) -> io::Result<WritePathOutcome>
where
    P: AsRef<Path>,
{
    try_write_string_into_file_with_outcome(file_name, &ts.to_string())
}
#[allow(clippy::single_call_fn)] // keeps rustfmt-trigger policy in one reusable decision helper
const fn should_run_rustfmt(
    format_with_cargofmt: FormatWithCargofmt,
    wr_outcome: &WritePathOutcome,
) -> bool {
    wr_outcome.is_changed() && matches!(format_with_cargofmt, FormatWithCargofmt::True)
}
pub fn try_mb_write_ts_into_file<P>(
    should_write_ts_into_file: ShouldWriteTsIntoFile,
    file_name: P,
    ts: &Ts2,
    format_with_cargofmt: &FormatWithCargofmt,
) -> io::Result<()>
where
    P: AsRef<Path>,
{
    if !should_write_ts_flag(should_write_ts_into_file) {
        return Ok(());
    }
    let wr_outcome = try_write_ts_into_file(file_name, ts)?;
    if should_run_rustfmt(*format_with_cargofmt, &wr_outcome) {
        try_run_rustfmt(wr_outcome.path())?;
    }
    Ok(())
}
pub fn mb_write_ts_into_file<P>(
    should_write_ts_into_file: ShouldWriteTsIntoFile,
    file_name: P,
    ts: &Ts2,
    format_with_cargofmt: &FormatWithCargofmt,
) where
    P: AsRef<Path>,
{
    panic_if_err(
        try_mb_write_ts_into_file(
            should_write_ts_into_file,
            file_name,
            ts,
            format_with_cargofmt,
        ),
        |er| format!("5ecc3880:{er}"),
    );
}
#[cfg(test)]
mod tests {
    use super::{
        FormatWithCargofmt, ShouldWriteTsIntoFile, mb_write_ts_into_file, should_write_ts_flag,
        try_mb_write_ts_into_file,
    };
    use crate::rs_file_path::rs_file_path;
    use crate::test_hlp::{assert_file_content, cleanup_test_file, test_path};
    use proc_macro2::TokenStream as Ts2;
    use std::fs::{metadata, write};
    #[test]
    fn mb_write_ts_into_file_skips_when_flag_is_false() {
        let base = test_path("macros_helpers_skip");
        let path = rs_file_path(&base);
        let ts: Ts2 = "struct SkipWrite;".parse().expect("5994e7e2");
        mb_write_ts_into_file(
            ShouldWriteTsIntoFile::False,
            &base,
            &ts,
            &FormatWithCargofmt::False,
        );
        let _er = metadata(&path).expect_err("7be5f201");
    }
    #[test]
    fn mb_write_ts_into_file_writes_tokens_when_flag_is_true() {
        let base = test_path("macros_helpers_write");
        let path = rs_file_path(&base);
        let ts: Ts2 = "struct DidWrite ;".parse().expect("6c20f49a");
        let expected = ts.to_string();
        mb_write_ts_into_file(
            ShouldWriteTsIntoFile::True,
            &base,
            &ts,
            &FormatWithCargofmt::False,
        );
        assert_file_content(&path, &expected);
        cleanup_test_file(path);
    }
    #[test]
    fn should_write_ts_flag_maps_true_and_false_flags() {
        assert!(!should_write_ts_flag(ShouldWriteTsIntoFile::False));
        assert!(should_write_ts_flag(ShouldWriteTsIntoFile::True));
    }
    #[test]
    fn try_mb_write_ts_into_file_writes_tokens_when_enabled() {
        let base = test_path("macros_helpers_try_write");
        let path = rs_file_path(&base);
        let ts: Ts2 = "struct TryDidWrite ;".parse().expect("f771ac2d");
        let expected = ts.to_string();
        try_mb_write_ts_into_file(
            ShouldWriteTsIntoFile::True,
            &base,
            &ts,
            &FormatWithCargofmt::False,
        )
        .expect("6fee9f6f");
        assert_file_content(&path, &expected);
        cleanup_test_file(path);
    }
    #[test]
    fn try_mb_write_ts_into_file_accepts_path_input() {
        let base = test_path("macros_helpers_try_write_path");
        let path = rs_file_path(&base);
        let ts: Ts2 = "struct PathInput ;".parse().expect("f9b0cd83");
        let expected = ts.to_string();
        try_mb_write_ts_into_file(
            ShouldWriteTsIntoFile::True,
            &base,
            &ts,
            &FormatWithCargofmt::False,
        )
        .expect("f341cde7");
        assert_file_content(&path, &expected);
        cleanup_test_file(path);
    }
    #[test]
    fn try_mb_write_ts_into_file_formats_when_rustfmt_enabled() {
        let base = test_path("macros_helpers_try_run_rustfmt");
        let path = rs_file_path(&base);
        write(&path, "struct B;").expect("7091840d");
        let ts: Ts2 = "struct A ;".parse().expect("0f30ca53");
        try_mb_write_ts_into_file(
            ShouldWriteTsIntoFile::True,
            &base,
            &ts,
            &FormatWithCargofmt::True,
        )
        .expect("00a995a4");
        assert_file_content(&path, "struct A;\n");
        cleanup_test_file(path);
    }
}
