use std::mem::MaybeUninit;
use winapi::um::winuser;
use {std::sync::mpsc, tray_item::TrayItem};

pub struct TrayPart;

pub enum Message {
    Quit,
    About,
}

impl TrayPart {
    pub(crate) const LINK_TO_REPOSITORY: &'static str = "https://github.com/jestsi/TranslateUncorrectLayout#-this-program-is-designed-to-translate-text-into-the-correct-layout-when-typing-is-incorrect-";

    pub fn start_listening_events() {
        let mut tray = TrayItem::new("GGTranslateUncorrectedLayout", "tray-icon-name").unwrap();

        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();

        tray.add_menu_item("About", move || {
            tx.send(Message::About).unwrap();
        })
        .unwrap();

        tray.add_menu_item("Quit", move || {
            tx1.send(Message::Quit).unwrap();
        })
        .unwrap();
        std::thread::spawn(move || {
            rx.iter().for_each(|m| match m {
                Message::Quit => std::process::exit(0),
                Message::About => open::that(Self::LINK_TO_REPOSITORY).unwrap(),
                _ => (),
            });
        }).join().unwrap();
        
    }
}
