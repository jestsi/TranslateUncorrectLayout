pub mod keyboard_output {
    use keybd_event::KeyBondingInstance;
    use keybd_event::KeyboardKey::{KeyA, KeyC, KeyV};

    pub struct KbOutput {
        pub insert: KeyBondingInstance,
        pub select_all: KeyBondingInstance,
        pub copy: KeyBondingInstance,
    }

    impl Default for KbOutput {
        fn default() -> Self {
            let mut exit: KbOutput = KbOutput {
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
}
