use std::collections::HashMap;
use std::fs::read_to_string;
use winput::{Vk};

// shortcut for invoke translate
pub const ADDITIONAL_KEY: Vk = Vk::Alt;
pub const KEY: Vk = Vk::T;

pub fn get_keys() -> HashMap<char, char> {
    return serde_json::from_str(
        &*read_to_string("culture_info.json").unwrap()).unwrap();
}

pub struct KeysHandlers {
    pub key: Vk,
    pub additional_key: Vk 
}

pub fn args_handler(args: &mut Vec<String>) -> Result<KeysHandlers, &'static str> {
    let mut exit = KeysHandlers{additional_key: ADDITIONAL_KEY, key: KEY};
    if args.len() <= 1{ return Ok(exit) }
    
    let mut k_v : HashMap<String, String> = HashMap::new();
    let mut prev_val : &String = &String::new();
    
    for (i,str) in args.iter().enumerate() {
        if i == 0 { continue }
        if i%2 == 0 && args.len() > i {
            prev_val = &str;
        } 
        k_v.insert(prev_val.clone().to_owned(), str.clone().to_owned());
    }
    
    for (key, val) in k_v {
        match key.to_lowercase().as_str() {
            "--addit_key" => exit.additional_key = match what_is_additional_key(&val) {
                Ok(x) => x,
                Err(_) => return Ok(exit)
            },
            _ => (),
        }
    }
    Ok(exit)
}

fn what_is_additional_key(kb_key: &String) -> Result<Vk, &'static str> {
    return match kb_key.to_lowercase().as_str() {
        "alt" => Ok( Vk::Alt),
        "ctrl" | "control" => Ok(Vk::Control),
        "shift" => Ok(Vk::Shift),
        "win" | "windows" => Ok(Vk::RightWin),
        _ => Err("Not have that key"),
    }
}