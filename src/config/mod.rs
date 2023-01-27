mod console_handler;

pub mod config_app {
    use winput::Vk;
    use std::env::args;
    use std::path::Path;
    use std::collections::HashMap;
    use crate::hide_console_window;
    use std::fs::{read_to_string, File};

    pub struct KeysHandlers {
        pub key: Vk,
        pub special_key: Vk,
    }

    pub struct Config {
        pub keys_handler: KeysHandlers,
        pub culture_info_path: String,
        pub culture_info: Option<HashMap<char, char>>,
        pub hide_console: bool,
        pub one_key: bool,
        generate_culture_info: bool,
    }

    impl KeysHandlers {
        pub const SPECIAL_KEY: Vk = Vk::Alt;
        pub const KEY: Vk = Vk::T;
    }

    impl Config {
        pub const CULTURE_INFO_PATH: &'static str = "culture_info.json";
        pub const CULTURE_INFO_DEFAULT: &'static str =
            include_str!("../../assets/culture_info.json");

        pub fn new(key: Vk, sp_key: Vk) -> Self {
            Self {
                culture_info_path: Self::CULTURE_INFO_PATH.parse().unwrap(),
                keys_handler: KeysHandlers {
                    key,
                    special_key: sp_key,
                },
                culture_info: None,
                hide_console: true,
                generate_culture_info: true,
                one_key: false,
            }
        }

        pub fn setting_config(&mut self) -> Result<(), &'static str> {
            let args: Vec<String> = args().collect();

            if args.len() <= 1 {
                self.culture_info = Some(self.get_keys()?);
                hide_console_window(self.hide_console);
                return Ok(());
            }
            let dictionary_args = Self::get_formatted_dictionary(&args);

            for (key, value) in dictionary_args {
                match key.to_lowercase().as_str() {
                    "--sp-key" | "-spk" => {
                        self.keys_handler.special_key =
                            Self::what_is_special_key(&value).unwrap_or(KeysHandlers::SPECIAL_KEY)
                    }
                    "--key" | "-k" => {
                        self.keys_handler.key =
                            Self::what_is_key(&value).unwrap_or(KeysHandlers::KEY)
                    }
                    "--culture-file" | "-cf" => self.if_change_path_culture_file(&value)?,
                    "--console-hide" | "-ch" => self.hide_console = value == "true",
                    "--culture-generate" | "-cg" => self.generate_culture_info = value == "true",
                    "--one-key" | "-ok" => self.one_key = value == "true",
                    _ => (),
                }
            }

            if self.culture_info.is_none() {
                self.culture_info = Some(self.get_keys()?);
            }
            hide_console_window(self.hide_console);
            Ok(())
        }

        fn if_change_path_culture_file(&mut self, path: &str) -> Result<(), &'static str> {
            let path_ = Path::new(path);
            if path_.exists() {
                self.culture_info_path = path.to_string();
            } else if self.generate_culture_info {
                self.culture_info_path = "./".to_string();
            } else {
                self.culture_info_path = String::new();
            }
            self.culture_info = Option::from(self.get_keys()?);
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
                "alt" | "menu" => Ok(Vk::Alt),
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

        fn get_keys(&self) -> Result<HashMap<char, char>, &'static str> {
            let error_msg = "Error read culture file";
            let culture_info = match read_to_string(&self.culture_info_path) {
                Ok(ok) => ok,
                Err(_) => {
                    self.show_error_msg(
                        "Error read culture file, will be use default culture info",
                        std::time::Duration::from_secs(3),
                    );
                    Self::generate_culture_info(self.generate_culture_info)?
                }
            };
            match serde_json::from_str(&culture_info) {
                Ok(ok) => Ok(ok),
                Err(_) => Err(error_msg),
            }
        }

        fn generate_culture_info(generate_file: bool) -> Result<String, &'static str> {
            if generate_file {
                let mut _culture_file =
                    File::create("culture_info.json").expect("Error create culture_info.json");
                let x: HashMap<char, char> =
                    serde_json::from_str(Self::CULTURE_INFO_DEFAULT).unwrap();

                serde_json::to_writer_pretty(_culture_file, &x)
                    .expect("Error write to generated file info");
            }
            Ok(Self::CULTURE_INFO_DEFAULT.to_string())
        }
    }
}
