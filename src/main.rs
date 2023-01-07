mod config;

extern crate clipboard;
extern crate keybd_event;
extern crate core;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::collections::HashMap;

use keybd_event::KeyBondingInstance;
use keybd_event::KeyboardKey::{KeyA, KeyC, KeyV};

use std::thread::sleep;
use std::time::Duration;

pub struct KB {
    pub insert: KeyBondingInstance,
    pub select_all: KeyBondingInstance,
    pub copy: KeyBondingInstance,
}

impl KB {
    pub fn init() -> KB {
        let mut exit: KB = KB {
            insert: KeyBondingInstance::new().unwrap(),
            copy: KeyBondingInstance::new().unwrap(),
            select_all: KeyBondingInstance::new().unwrap(),
        };

        exit.copy.has_ctrl(true);
        exit.insert.has_ctrl(true);
        exit.select_all.has_ctrl(true);

        exit.copy.add_key(KeyC);
        exit.insert.add_key(KeyV);
        exit.select_all.add_key(KeyA);

        exit
    }
}

fn main() {
    let mut clip_prov: ClipboardContext =
        ClipboardProvider::new().expect("init clip provider failed!");

    let mut args : Vec<String> = std::env::args().collect();
    println!("args - {:#?}", args);
    
    let keys_use_handler = config::args_handler(&mut args);
    
    let mut kb = KB::init();
    let sp_duration = Duration::from_millis(200);
    loop {
        if keys_use_handler.additional_key.is_down() && keys_use_handler.key.is_down() {
            sleep(sp_duration);
            kb.select_all.launching();
            sleep(sp_duration);
            kb.copy.launching();
            sleep(sp_duration);
            translate(&mut clip_prov, config::get_keys());
            sleep(sp_duration);
            kb.select_all.launching();
            sleep(sp_duration);
            kb.insert.launching();
            sleep(sp_duration);
        }
    }
}

fn translate(clip_provider: &mut ClipboardContext, keys: HashMap<char, char>) {
    let mut str_for_translate = match clip_provider.get_contents() {
        Ok(x) => x.to_lowercase(),
        Err(_) => panic!("error get content"),
    };

    str_for_translate = str_for_translate
        .chars()
        .map(|ch| *keys.get(&ch).unwrap_or(&ch))
        .collect();

    match clip_provider.set_contents(str_for_translate) {
        Ok(_) => {},
        Err(_) => panic!("error set content")
    }
}
