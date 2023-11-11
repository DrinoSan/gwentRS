use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Debug)]
struct Creature {
    name: String,
    mana_cost: i32,
    damage_points: i32,
    life_points: i32,
    mana_drain: bool,
    shield: bool,
}

pub struct Config;

impl Config {
    pub fn validate_config(config: &serde_json::Value) -> bool {
        let configuration = config.as_object().unwrap();

        return Self::check_type_of_cards(configuration)
            && Self::check_card_count(configuration)
            && Self::check_name_length(configuration)
            && Self::check_equals(configuration)
            && Self::check_spells( configuration );

    }

    fn check_type_of_cards(config: &Map<String, Value>) -> bool {
        return config.contains_key("Creatures") && config.contains_key("Spells");
    }

    fn check_card_count(config: &Map<String, Value>) -> bool {
        return config["Creatures"].as_array().unwrap().len()
            + config["Spells"].as_array().unwrap().len()
            > 10;
    }

    fn check_name_length(config: &Map<String, Value>) -> bool {
        for creature in config["Creatures"].as_array().unwrap() {
            if creature["name"].as_str().unwrap().len() > 8 {
                return false;
            }
        }

        return true;
    }

    fn check_equals(config: &Map<String, Value>) -> bool {
        // let creature_arr = config["Creatures"].as_array().unwrap();
        // let creature_vec = serde_json::from_value::<Vec<Creature>>(creature_arr.clone());

        let creature_arr = config["Creatures"].as_array().unwrap();
        let creature_vec: Vec<Creature> = serde_json::from_value(serde_json::Value::Array(creature_arr.clone())).unwrap();

        for i in 0..creature_vec.len() {
            for j in (i + 1)..creature_vec.len() {
                if creature_vec[i].name == creature_vec[j].name {
                    if creature_vec[i].mana_cost != creature_vec[j].mana_cost
                        || creature_vec[i].damage_points != creature_vec[j].damage_points
                        || creature_vec[i].life_points != creature_vec[j].life_points
                        || creature_vec[i].shield != creature_vec[j].shield
                        || creature_vec[i].mana_drain != creature_vec[j].mana_drain
                    {
                        return false;
                    }
                }
            }
        }
        return true;
    }


    fn check_spells(config: &Map<String, Value>) -> bool {
        
    }
}

