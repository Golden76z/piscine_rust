use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        let short_hand = "-".to_string()
            + &name
                .chars()
                .next()
                .expect("ERROR: name string empty")
                .to_string();
        let long_hand = "--".to_string() + name;
        Flag {
            short_hand: short_hand,
            long_hand: long_hand,
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        // Look up the callback by input flag
        if let Some(callback) = self.flags.get(input) {
            if argv.len() < 2 {
                return Err("Not enough arguments".to_string());
            }
            // Call callback with first two args, map ParseFloatError to string
            callback(argv[0], argv[1]).map_err(|e| e.to_string())
        } else {
            Err(format!("Flag '{}' not found", input))
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a1: f64 = a.parse()?;
    let b1: f64 = b.parse()?;

    Ok((a1 / b1).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a1: f64 = a.parse()?;
    let b1: f64 = b.parse()?;

    Ok((a1 % b1).to_string())
}
