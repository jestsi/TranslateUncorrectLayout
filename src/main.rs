use std::thread::sleep;
use std::time::Duration;

fn main() {
    if let Err(e) = translate_app::run() {
        translate_app::hide_console_window(false);
        eprintln!("Application error [{}]", e);
        sleep(Duration::from_secs(5));
    }
}
