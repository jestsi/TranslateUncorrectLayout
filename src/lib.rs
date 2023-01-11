mod config;
mod kb_output;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::collections::HashMap;
use std::thread::sleep;

use crate::config::{Config, KeysHandlers};
use crate::kb_output::KbOutput;
use winput::message_loop;

const SLP_DURATION: std::time::Duration = std::time::Duration::from_millis(200);

pub fn run() -> Result<(), &'static str> {
    let clipbd_context: ClipboardContext =
        ClipboardProvider::new().expect("Clipboard Context create fail!");
    let mut cfg = Config::new(KeysHandlers::KEY, KeysHandlers::ADDITIONAL_KEY);
    cfg.setting_config();

    start_loop(&mut cfg, clipbd_context)
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
            if cfg.keys_handler.additional_key.is_down() && cfg.keys_handler.key.is_down() {
                sleep(SLP_DURATION);
                kb.select_all.launching();
                sleep(SLP_DURATION);
                kb.copy.launching();
                sleep(SLP_DURATION);

                let string_for_translate = clipbd_context
                    .get_contents()
                    .expect("Error get content from clipboard")
                    .to_lowercase();
                let translated_result =
                    translate(string_for_translate, cfg.culture_info.as_mut().unwrap());

                if let Err(e) = translated_result {
                    match e {
                        "Error get content from clipboard" | "Error set content in clipboard" => {
                            continue
                        }
                        _ => return Err(e),
                    }
                } else {
                    clipbd_context
                        .set_contents(translated_result.unwrap())
                        .expect("Error set content in clipboard")
                }

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
