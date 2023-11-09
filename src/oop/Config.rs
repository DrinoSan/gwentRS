pub struct Config;

impl Config {
    pub fn validate_config(config: &serde_json::Value) -> bool {
        // return Self::check_type_of_cards(config)
        //     && Self::check_card_count(config)
        //     && Self::check_name_length(config)
        //     && Self::check_equals(config);

        let v = config.as_object().unwrap();
        println!("OOOOOOOO: {:?}", v["Spells"]);
        true
    }

    fn check_type_of_cards(config: &serde_json::Value) -> bool {
        todo!()
    }

    fn check_card_count(config: &serde_json::Value) -> bool {
        todo!()
    }

    fn check_name_length(config: &serde_json::Value) -> bool {
        todo!()
    }

    fn check_equals(config: &serde_json::Value) -> bool {
        todo!()
    }
}

