use crate::hide_console_window;

impl super::config_app::Config {
    pub fn hide_console_window(hide: bool) {
        use winapi::um::wincon::GetConsoleWindow;
        use winapi::um::winuser::{ShowWindow, SW_HIDE, SW_SHOWNORMAL};

        let window = unsafe { GetConsoleWindow() };
        if !window.is_null() {
            unsafe {
                ShowWindow(window, if hide { SW_HIDE } else { SW_SHOWNORMAL });
            }
        }
    }

    pub fn show_error_msg(str: &str, dur: std::time::Duration) {
        hide_console_window(false);
        eprintln!("{}", str);
        std::thread::sleep(dur);
        hide_console_window(true);
    }
}
