use std::panic::set_hook;
pub fn panic_location() {
    set_hook(Box::new(move |panic_info| {
        if let Some(location) = panic_info.location() {
            eprintln!(
                "panic occurred in {}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            );
        } else {
            eprintln!("panic occurred but can't get location information...");
        }
    }));
}
