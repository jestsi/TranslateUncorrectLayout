use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;
use std::path::Path;
use winput::Vk;

pub struct KeysHandlers {
    pub key: Vk,
    pub special_key: Vk,
}
pub struct Config {
    pub keys_handler: KeysHandlers,
    pub culture_info_path: String,
    pub culture_info: Option<HashMap<char, char>>,
    pub hide_console: bool,
}

impl KeysHandlers {
    pub const SPECIAL_KEY: Vk = Vk::Alt;
    pub const KEY: Vk = Vk::T;
}
impl Config {
    pub const CULTURE_INFO_PATH: &'static str = "culture_info.json";

    pub fn new(key: Vk, sp_key: Vk) -> Self {
        Self {
            culture_info_path: Self::CULTURE_INFO_PATH.parse().unwrap(),
            keys_handler: KeysHandlers {
                key,
                special_key: sp_key,
            },
            culture_info: None,
            hide_console: true,
        }
    }

    pub fn setting_config(&mut self) -> Result<(), &'static str> {
        let args: Vec<String> = args().collect();

        if args.len() <= 1 {
            self.culture_info = Some(Self::get_keys(&self.culture_info_path)?);
            return Ok(());
        }
        let dictionary_args = Self::get_formatted_dictionary(&args);
        
        for (key, value) in dictionary_args {
            match key.to_lowercase().as_str() {
                "--sp-key" => self.keys_handler.special_key =
                        Self::what_is_special_key(&value).unwrap_or(KeysHandlers::SPECIAL_KEY),
                "--key" => self.keys_handler.key = Self::what_is_key(&value).unwrap_or(KeysHandlers::KEY),
                "--culture-file" => self.if_change_path_culture_file(&value)?,
                "--console-hide" => self.hide_console = value == "true",
                _ => (),
            }
        }
        
        Config::hide_console_window( self.hide_console);
        
        Ok(())
    }

    fn if_change_path_culture_file(&mut self, path: &str) -> Result<(), &'static str> {
        let path_ = Path::new(path);

        if !path_.is_file() {
            return Err("Isn`t file");
        }

        self.culture_info_path = path.to_owned();
        self.culture_info = Option::from(Self::get_keys(&self.culture_info_path)?);
        Ok(())
    }

    fn what_is_key(kb_key: &str) -> Result<Vk, &'static str> {
        let key_with_str = kb_key
            .to_uppercase()
            .chars()
            .next()
            .expect("Error get first element") as u8;
        
        if !(65..=90).contains(&key_with_str) {
            return Err("char is uncorrected");
        }

        unsafe { Ok(Vk::from_u8(key_with_str)) }
    }

    fn what_is_special_key(kb_key: &str) -> Result<Vk, &'static str> {
        match kb_key.to_lowercase().as_str() {
            "alt" => Ok(Vk::Alt),
            "ctrl" | "control" => Ok(Vk::Control),
            "shift" => Ok(Vk::Shift),
            "win" | "windows" => Ok(Vk::RightWin),
            _ => Err("Not have that sp key"),
        }
    }

    fn get_formatted_dictionary(args: &[String]) -> HashMap<String, String> {
        let mut exit_hash_map: HashMap<String, String> = HashMap::new();

        let mut previous_value = &String::new();
        for (index, str) in args.iter().enumerate() {
            if index == 0 {
                continue;
            }
            if index % 2 != 0 {
                previous_value = str;
            }
            exit_hash_map.insert(previous_value.clone().to_owned(), str.clone().to_owned());
        }

        exit_hash_map
    }

    fn get_keys(culture_info_path: &String) -> Result<HashMap<char, char>, &'static str> {
        let error_msg = "Error read culture file";
        let culture_info = match read_to_string(culture_info_path) {
            Ok(ok) => ok,
            Err(_) => return Err(error_msg),
        };
        match serde_json::from_str(&culture_info) {
            Ok(ok) => Ok(ok),
            Err(_) => Err(error_msg),
        }
    }
}
