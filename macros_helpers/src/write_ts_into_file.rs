use proc_macro2::TokenStream as Ts2;
use serde::Deserialize;
use std::{fs::File, io::Write, path::Path, process::Command};
#[derive(Debug, Clone, Copy)]
pub enum FormatWithCargofmt {
    False,
    True,
}
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum ShouldWriteTokenStreamIntoFile {
    False,
    True,
}
pub fn maybe_write_ts_into_file(
    should_write_ts_into_file: ShouldWriteTokenStreamIntoFile,
    file_name: &str,
    ts: &Ts2,
    format_with_cargofmt: &FormatWithCargofmt,
) {
    if matches!(
        should_write_ts_into_file,
        ShouldWriteTokenStreamIntoFile::True
    ) {
        let path_string = format!("{file_name}.rs");
        let path = Path::new(&path_string);
        {
            let mut file = File::create(path).expect("933f96b3");
            Write::write_all(&mut file, ts.to_string().as_bytes()).expect("a503bf88");
        };
        //no other way to format only one file. it formats all files in project
        if matches!(format_with_cargofmt, FormatWithCargofmt::True) {
            let status = Command::new("cargo")
                .arg("fmt")
                .arg("--")
                .arg(path)
                .status()
                .expect("5ecc3880");
            assert!(status.success(), "cargo fmt failed for {}", path.display());
        }
    }
}
