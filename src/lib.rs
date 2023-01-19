mod config;
mod kb_output;
mod console_handler;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::collections::HashMap;
use std::thread::sleep;

use crate::config::{Config, KeysHandlers};
use crate::kb_output::KbOutput;
use winput::message_loop;

const SLP_DURATION: std::time::Duration = std::time::Duration::from_millis(250);

pub fn run() -> Result<(), &'static str> {
    let clipbd_context: ClipboardContext =
        ClipboardProvider::new().expect("Clipboard Context create fail!");
    let mut cfg = Config::new(KeysHandlers::KEY, KeysHandlers::SPECIAL_KEY);
    cfg.setting_config()?;

    start_loop(&mut cfg, clipbd_context)
}

pub fn hide_console_window(hide: bool) {
    Config::hide_console_window(hide);
}

fn start_loop(cfg: &mut Config, mut clipbd_context: ClipboardContext) -> Result<(), &'static str> {
    let receiver = message_loop::start().unwrap();
    let mut kb = KbOutput::default();

    loop {
        if let message_loop::Event::Keyboard {
            action: winput::Action::Press,
            ..
        } = receiver.next_event()
        {
            if cfg.keys_handler.special_key.is_down() && cfg.keys_handler.key.is_down() {
                sleep(SLP_DURATION);
                kb.select_all.launching();
                sleep(SLP_DURATION);
                kb.copy.launching();
                sleep(SLP_DURATION);

                let string_for_translate = clipbd_context
                    .get_contents()
                    .unwrap_or_default()
                    .to_lowercase();

                let translated_result = if cfg.culture_info.is_some() {
                    translate(string_for_translate, cfg.culture_info.as_ref().unwrap())?
                } else { String::new() };

                clipbd_context.set_contents(translated_result).unwrap();

                sleep(SLP_DURATION);
                kb.select_all.launching();
                sleep(SLP_DURATION);
                kb.insert.launching();
                sleep(SLP_DURATION);
            }
        }
    }
}

fn translate(
    str_for_translate: String,
    keys: &HashMap<char, char>,
) -> Result<String, &'static str> {
    if str_for_translate.is_empty() { return Ok(String::new()) }
    let count_x_chars = str_for_translate
        .chars()
        .filter(|x| !x.is_alphabetic() || x.is_ascii_alphabetic())
        .count();
    
    let mut keys_mut = keys.clone();
    if count_x_chars != str_for_translate.len() {
        keys_mut = inverse_hashmap(&keys_mut);
    }

    Ok(str_for_translate
        .chars()
        .map(|ch| *keys_mut.get(&ch).unwrap_or(&ch))
        .collect())
}

fn inverse_hashmap(keys: &HashMap<char, char>) -> HashMap<char, char> {
    let mut exit = HashMap::new();
    for (key, value) in keys {
        exit.insert(*value, *key);
    }
    exit.to_owned()
}
