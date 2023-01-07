use std::collections::HashMap;
use winput::{Vk};


// shortcut for invoke translate
pub const ADDITIONAL_KEY: Vk = Vk::Alt;
pub const KEY: Vk = Vk::T;

pub fn get_keys() -> HashMap<char, char> {
    return HashMap::from([
        ('q', 'й'),
        ('w', 'ц'),
        ('e', 'у'),
        ('r', 'к'),
        ('t', 'е'),
        ('y', 'н'),
        ('u', 'г'),
        ('i', 'ш'),
        ('o', 'щ'),
        ('p', 'з'),
        ('[', 'х'),
        (']', 'ъ'),
        ('a', 'ф'),
        ('s', 'ы'),
        ('d', 'в'),
        ('f', 'а'),
        ('g', 'п'),
        ('h', 'р'),
        ('j', 'о'),
        ('k', 'л'),
        ('l', 'д'),
        (';', 'ж'),
        ('\'', 'э'),
        ('\\', '\\'),
        ('z', 'я'),
        ('x', 'ч'),
        ('c', 'с'),
        ('v', 'м'),
        ('b', 'и'),
        ('n', 'т'),
        ('m', 'ь'),
        (',', 'б'),
        ('.', 'ю'),
        ('/', '.'),
    ]);
}

pub struct KeysHandlers {
    pub key: Vk,
    pub additional_key: Vk 
}

pub fn args_handler(args: &mut Vec<String>) -> KeysHandlers {
    args.remove(0);
    let mut exit = KeysHandlers{additional_key: ADDITIONAL_KEY, key: KEY};
    
    if args.is_empty() { return exit }
    
    let mut k_v : HashMap<String, String> = HashMap::new();
    let mut prev_val : &String = &String::new();
    
    for (i,str) in args.iter().enumerate() {
        if i%2 == 0 && args.len() > i {
            prev_val = &str;
        } 
        k_v.insert(prev_val.clone().to_owned(), str.clone().to_owned());
    }
    
    for (key, val) in k_v {
        match key.as_str() {
            "--addit_key" => {exit.additional_key = what_is_additional_key(&val) },
            _ => (),
        }
    }
    exit
}

fn what_is_additional_key(kb_key: &String) -> Vk {
    match kb_key.as_str() {
        "alt" => Vk::Alt,
        "ctrl" | "control" => Vk::Control,
        "shift" => Vk::Shift,
        "win" | "windows" => Vk::RightWin,
        _ => panic!("Not find that key"),
    }
}