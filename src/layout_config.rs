use hson::Value;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use crate::screen::Screen;

pub fn load(path: &str) -> Result<HashMap<String, Screen>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut hjson: Map<String, Value> = serde_hjson::from_reader(reader)?;

    let mut screens = HashMap::new();

    for (screen_name, screen_val) in hjson.iter() {
        let screen = parse_screen(screen_name, screen_val);
        screens.insert(screen_name, screen);
    }

    Ok(screens)
}

fn parse_screen(name: &str, val: String) { }
