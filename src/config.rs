use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;
use std::path::Path;
use winput::Vk;

pub struct KeysHandlers {
    pub key: Vk,
    pub additional_key: Vk,
}
pub struct Config {
    pub keys_handler: KeysHandlers,
    pub culture_info_path: String,
    pub culture_info: Option<HashMap<char, char>>
}

impl KeysHandlers {
    pub const ADDITIONAL_KEY: Vk = Vk::Alt;
    pub const KEY: Vk = Vk::T;
}
impl Config {
    pub const CULTURE_INFO_PATH: &'static str = "culture_info.json";
    
    pub fn new(key: Vk, sp_key: Vk) -> Self {
        Self {
            culture_info_path: Self::CULTURE_INFO_PATH.parse().unwrap(),
            keys_handler: KeysHandlers { key, additional_key: sp_key },
            culture_info: None
        }
    }
    
    pub fn setting_config(&mut self) {
        let args : Vec<String> = args().collect();
        
        if args.len() <= 1 {
            self.culture_info = Some(Self::get_keys(&self.culture_info_path));
            return; 
        }
        let dictionary_args = Self::get_format_dictanary(&args);
        for (key, value) in dictionary_args {
            match key.to_lowercase().as_str() {
                "--sp-key" => self.keys_handler.additional_key = Self::what_is_special_key(&value).expect("Error on set sp key"),
                "--culture-file" => self.if_change_path_culture_file(&value).expect("Error change path culture file"),
                _ => (),
            }
        }
    }

    fn if_change_path_culture_file(&mut self, path: &str) -> Result<(), &'static str> {
        let path_ = Path::new(path);
        path_.try_exists().expect("No such file exits");
        
        if !path_.is_file() { return Err("Isn`t file"); }
        
        self.culture_info_path = path.to_owned();
        self.culture_info = Option::from(Self::get_keys(&self.culture_info_path));
        Ok(())
    }
    
    fn what_is_special_key(kb_key: &str) -> Result<Vk, &'static str> {
        match kb_key.to_lowercase().as_str() {
            "alt" => Ok( Vk::Alt ),
            "ctrl" | "control" => Ok(Vk::Control),
            "shift" => Ok(Vk::Shift),
            "win" | "windows" => Ok(Vk::RightWin),
            _ => Err("Not have that key"),
        }
    }
    
    fn get_format_dictanary(args: &[String]) -> HashMap<String, String> {
        let mut exit_hash_map: HashMap<String, String> = HashMap::new();

        let mut previous_value = &String::new();
        for (index, str) in args.iter().enumerate() {
            if index == 0 { continue;}
            if index % 2 != 0  { previous_value = str;}
            exit_hash_map.insert(previous_value.clone().to_owned(), str.clone().to_owned());
        }

        exit_hash_map
    }
    
    fn get_keys(culture_info_path: &String) -> HashMap<char, char> {
        serde_json::from_str(&read_to_string(culture_info_path).expect("Error read culture file")).unwrap()
    }
}