use crate::config::Config;

impl Config {
    pub fn hide_console_window(hide: bool) {
        use winapi::um::wincon::GetConsoleWindow;
        use winapi::um::winuser::{ShowWindow, SW_HIDE, SW_SHOWNORMAL};

        let window = unsafe {GetConsoleWindow()};
        if !window.is_null(){
            unsafe {
                ShowWindow(window, if hide {SW_HIDE} else { SW_SHOWNORMAL });
            }
        }
    }
}
