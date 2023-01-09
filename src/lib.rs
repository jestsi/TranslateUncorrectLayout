mod config;
mod kb_output;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::collections::HashMap;
use std::thread::sleep;

use winput::{message_loop};
use crate::config::Config;
use crate::kb_output::KbOutput;

const SLP_DURATION: std::time::Duration = std::time::Duration::from_millis(200);

pub fn run() -> Result<(), &'static str> {
    let clipbd_context: ClipboardContext =
        ClipboardProvider::new().expect("Clipboard Context create fail!");
    let cfg =
        config::args_handler(&mut std::env::args().collect()).expect("Error get keys shortcut");
    let kb = KbOutput::default();
    start_loop(cfg, kb, clipbd_context).expect_err("Error on start loop");
    Ok(())
}

fn start_loop(cfg: Config, mut kb: KbOutput, mut clipbd_context: ClipboardContext) -> Result<(), &'static str> {
    let translate_keys: HashMap<char, char> = config::get_keys(cfg.culture_info_path.as_str());
    let receiver = message_loop::start().unwrap();
    loop {
        if let message_loop::Event::Keyboard {action: winput::Action::Press, ..} = receiver.next_event() {
            if cfg.keys_handler.additional_key.is_down() && cfg.keys_handler.key.is_down() {
                sleep(SLP_DURATION);
                kb.select_all.launching();
                sleep(SLP_DURATION);
                kb.copy.launching();
                sleep(SLP_DURATION);
                
                let string_for_translate = clipbd_context.get_contents().expect("Error get content from clipboard").to_lowercase();
                let translated_result = translate(string_for_translate, &translate_keys);
                
                if let Err(e) = translated_result
                {
                    match e {
                        "Error get content from clipboard" | "Error set content in clipboard" => continue,
                        _ => return Err(e),
                    }
                } else { 
                    clipbd_context.set_contents( translated_result.unwrap() ).expect("Error set content in clipboard") 
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

fn translate(str_for_translate: String, keys: &HashMap<char, char>) -> Result<String, &'static str> {
    Ok(str_for_translate
        .chars()
        .map(|ch| *keys.get(&ch).unwrap_or(&ch))
        .collect())
}
