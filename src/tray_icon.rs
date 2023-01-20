use std::mem::MaybeUninit;
use tokio::sync::mpsc;
use tray_item::TrayItem;
use winapi::um::winuser;

pub struct TrayPart;

pub enum Message {
    Quit,
    About,
}

impl TrayPart {
    pub(crate) const LINK_TO_REPOSITORY: &'static str = "https://github.com/jestsi/TranslateUncorrectLayout#-this-program-is-designed-to-translate-text-into-the-correct-layout-when-typing-is-incorrect-";

    pub fn start_listen_tray_events() {
        let mut tray = TrayItem::new("GGTranslateUncorrectedLayout", "tray-icon-name").unwrap();

        tray.add_menu_item("About", move || {
            tx.send(Message::About).unwrap();
        })
        .unwrap();

        tray.add_menu_item("Quit", move || {
            std::process::exit(0);
        })
        .unwrap();
    }
}
