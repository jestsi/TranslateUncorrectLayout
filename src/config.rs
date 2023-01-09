use std::collections::HashMap;
use std::fs::{read_to_string};
use std::path::Path;
use winput::{Vk};

pub struct KeysHandlers {
    pub key: Vk,
    pub additional_key: Vk
}
pub struct Config {
    pub keys_handler: KeysHandlers,
    pub culture_info: String,
}

impl Default for KeysHandlers {
    fn default() -> Self {
        Self {
            key: KEY,
            additional_key: ADDITIONAL_KEY
        }
    }
}
impl Default for Config {
    fn default() -> Config {
        Config {
            keys_handler: KeysHandlers::default(),
            culture_info: CULTURE_INFO_PATH.to_owned(),
        }
    }
}

// shortcut for invoke translate
pub const ADDITIONAL_KEY: Vk = Vk::Alt;
pub const KEY: Vk = Vk::T;
pub const CULTURE_INFO_PATH: &str = "culture_info.json";

pub fn get_keys(path_cult_file: &str) -> HashMap<char, char> {
    serde_json::from_str(
        &read_to_string(path_cult_file).unwrap()).unwrap()
}

pub fn args_handler(args: &mut Vec<String>) -> Result<Config, &'static str> {
    let mut exit = Config::default();
    if args.len() <= 1{ return Ok(exit) }
    
    let mut k_v : HashMap<String, String> = HashMap::new();
    let mut prev_val : &String = &String::new();
    
    for (i,str) in args.iter().enumerate() {
        if i == 0 { continue }
        if i%2 != 0 && args.len() > i {
            prev_val = str;
        } 
        k_v.insert(prev_val.clone().to_owned(), str.clone().to_owned());
    }
    
    for (key, val) in k_v {
        match key.to_lowercase().as_str() {
            "--addit_key" => what_is_additional_key(&val, &mut exit.keys_handler)?,
            "--culture_file" => if_change_path_culture_file(&val, &mut exit)?,
            _ => (),
        }
    }
    Ok(exit)
}

fn if_change_path_culture_file(path: &str, config: &mut Config) -> Result<(), &'static str> {
    let path_ = Path::new(path);
    path_.try_exists().expect("No such file exits");
    if !path_.is_file() { return Err("Isn`t file") };
    config.culture_info = path.to_owned();
    Ok(())
}

fn what_is_additional_key(kb_key: &str, exit_val: &mut KeysHandlers) -> Result<(), &'static str> {
    exit_val.additional_key = match kb_key.to_lowercase().as_str() {
        "alt" => Vk::Alt,
        "ctrl" | "control" => Vk::Control,
        "shift" => Vk::Shift,
        "win" | "windows" => Vk::RightWin,
        _ => return Err("Not have that key"),
    };
    Ok(())
}