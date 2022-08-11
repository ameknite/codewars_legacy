use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref MORSE_CODE: HashMap<String, String> = {
        let mut map = HashMap::new();
        map.insert(".-".to_string(), "A".to_string());
        map.insert("-...".to_string(), "B".to_string());
        map.insert("-.-.".to_string(), "C".to_string());
        map.insert("-..".to_string(), "D".to_string());
        map.insert(".".to_string(), "E".to_string());
        map.insert("..-.".to_string(), "F".to_string());
        map.insert("--.".to_string(), "G".to_string());
        map.insert("....".to_string(), "H".to_string());
        map.insert("..".to_string(), "I".to_string());
        map.insert(".---".to_string(), "J".to_string());
        map.insert("-.-".to_string(), "K".to_string());
        map.insert(".-..".to_string(), "L".to_string());
        map.insert("--".to_string(), "M".to_string());
        map.insert("-.".to_string(), "N".to_string());
        map.insert("---".to_string(), "O".to_string());
        map.insert(".--.".to_string(), "P".to_string());
        map.insert("--.-".to_string(), "Q".to_string());
        map.insert(".-.".to_string(), "R".to_string());
        map.insert("...".to_string(), "S".to_string());
        map.insert("-".to_string(), "T".to_string());
        map.insert("..-".to_string(), "U".to_string());
        map.insert("...-".to_string(), "V".to_string());
        map.insert(".--".to_string(), "W".to_string());
        map.insert("-..-".to_string(), "X".to_string());
        map.insert("-.--".to_string(), "Y".to_string());
        map.insert("--..".to_string(), "Z".to_string());
        map
    };
}
