pub(crate) fn panic_if_err<T, E>(res: Result<T, E>, mk_panic_msg: impl FnOnce(E) -> String) -> T {
    match res {
        Ok(ok_v) => ok_v,
        Err(er) => panic!("{}", mk_panic_msg(er)),
    }
}
#[cfg(test)]
mod tests {
    use super::panic_if_err;
    use std::panic::catch_unwind;
    #[test]
    fn panic_if_err_returns_ok_value() {
        let value = panic_if_err::<u8, u16>(Ok(7), |_| String::from("unused"));
        assert_eq!(value, 7);
    }
    #[test]
    fn panic_if_err_panics_for_error() {
        let panic_res =
            catch_unwind(|| panic_if_err::<u8, u16>(Err(9), |er| format!("abc12345:{er}")));
        drop(panic_res.expect_err("3f86ac2d"));
    }
}
