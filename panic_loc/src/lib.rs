use std::{panic::set_hook, sync::Once};
static PANIC_HOOK_ONCE: Once = Once::new();
const PANIC_NO_LOCATION_MSG: &str = "panic occurred but can't get location information...";
pub fn panic_loc() {
    PANIC_HOOK_ONCE.call_once(|| {
        set_hook(Box::new(move |panic_info| {
            if let Some(location) = panic_info.location() {
                eprintln!(
                    "panic occurred in {}:{}:{}",
                    location.file(),
                    location.line(),
                    location.column()
                );
            } else {
                eprintln!("{PANIC_NO_LOCATION_MSG}");
            }
        }));
    });
}
#[cfg(test)]
mod tests {
    use super::{PANIC_NO_LOCATION_MSG, panic_loc};
    #[test]
    fn panic_loc_can_be_called_multiple_times() {
        panic_loc();
        panic_loc();
    }
    #[test]
    fn panic_no_location_message_is_stable() {
        assert_eq!(
            PANIC_NO_LOCATION_MSG,
            "panic occurred but can't get location information..."
        );
    }
}
