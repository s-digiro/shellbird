use json::JsonValue;
use json::object::Object;

use unicode_truncate::Alignment;

use std::collections::HashMap;
use std::fs;
use std::error::Error;

use crate::components::*;
use crate::screen::Screen;
use crate::color::Color;

#[cfg(test)]
mod tests;

pub fn load(path: &str) -> Result<HashMap<String, Screen>, Box<dyn Error>> {
    let file_contents = match fs::read_to_string(path) {
        Ok(fc) => fc,
        Err(e) => return Err(Box::new(e)),
    };

    let val = match json::parse(&file_contents) {
        Ok(x) => x,
        Err(e) => return Err(Box::new(e)),
    };

    let mut screens = HashMap::new();

    if let JsonValue::Object(obj) = val {
        for (screen_name, screen_val) in obj.iter() {
            if let Some(screen) = parse_screen(screen_name, screen_val) {
                screens.insert(screen_name.to_string(), screen);
            }
        }
    }

    Ok(screens)
}

fn parse_screen(name: &str, val: &JsonValue) -> Option<Screen> {
    if let Some(base) = parse_component(val) {
        Some(Screen::new(name, base))
    } else {
        None
    }
}

fn parse_component(val: &JsonValue) -> Option<Components> {
    if let JsonValue::Object(obj) = val {
        if let Some(component_type) = obj.get("component") {
            if let Some(component_type) = component_type.as_str() {
                match component_type {
                    "HorizontalSplitter" => parse_horizontal_splitter(obj),
                    "EmptySpace" => Some(parse_empty_space(obj)),
                    "VerticalSplitter" => parse_vertical_splitter(obj),
                    "PlaceHolder" => Some(parse_place_holder(obj)),
                    "TagDisplay" => Some(parse_tag_display(obj)),
                    "TitleDisplay" => Some(parse_title_display(obj)),
                    "Queue" => Some(parse_queue(obj)),
                    "PlaylistMenu" => Some(parse_playlist_menu(obj)),
                    "TrackMenu" => Some(parse_track_menu(obj)),
                    "TagMenu" => Some(parse_tag_menu(obj)),
                    "StyleMenu" => Some(parse_style_menu(obj)),
                    _ => None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_style_menu(obj: &Object) -> Components {
    StyleMenu::enumed(
        parse_name(obj, "StyleMenu"),
        parse_color(obj, "color"),
        parse_color(obj, "focus_color"),
        parse_parent(obj)
    )
}

fn parse_tag_menu(obj: &Object) -> Components {
    TagMenu::enumed(
        parse_name(obj, "TagMenu"),
        parse_color(obj, "color"),
        parse_color(obj, "focus_color"),
        parse_tag(obj, "Artist"),
        parse_parent(obj)
    )
}

fn parse_track_menu(obj: &Object) -> Components {
    TrackMenu::enumed(
        parse_name(obj, "TrackMenu"),
        parse_color(obj, "color"),
        parse_color(obj, "focus_color"),
        parse_parent(obj),
    )
}

fn parse_playlist_menu(obj: &Object) -> Components {
    PlaylistMenu::enumed(
        parse_name(obj, "PlaylistMenu"),
        parse_color(obj, "color"),
        parse_color(obj, "focus_color"),
    )
}

fn parse_queue(obj: &Object) -> Components {
    Queue::enumed(
        parse_name(obj, "Queue"),
        parse_color(obj, "color"),
        parse_color(obj, "focus_color"),
    )
}

fn parse_title_display(obj: &Object) -> Components {
    TitleDisplay::enumed(
        parse_name(obj, "TitleDisplay"),
        parse_color(obj, "color"),
        parse_alignment(obj),
    )
}

fn parse_tag_display(obj: &Object) -> Components {
    TagDisplay::enumed(
        parse_name(obj, "TagDisplay"),
        parse_color(obj, "color"),
        parse_alignment(obj),
        parse_tag(obj, "Artist"),
    )
}

fn parse_parent<'a>(obj: &'a Object) -> Option<String> {
    match parse_string(obj, "parent") {
        Some(s) => Some(s.to_string()),
        None => None,
    }
}

fn parse_tag<'a>(obj: &'a Object, def: &'a str) -> &'a str {
    match parse_string(obj, "tag") {
        Some(s) => s,
        None => def,
    }
}

fn parse_name<'a>(obj: &'a Object, def: &'a str) -> &'a str {
    match parse_string(obj, "name") {
        Some(s) => s,
        None => def,
    }
}

fn parse_string<'a>(obj: &'a Object, key: &str) -> Option<&'a str> {
    match obj.get(key) {
        Some(s) => s.as_str(),
        None => None,
    }
}

fn parse_place_holder(obj: &Object) -> Components {
    PlaceHolder::enumed(
        parse_name(obj, "PlaceHolder"),
        parse_color(obj, "color"),
    )
}

fn parse_empty_space(obj: &Object) -> Components {
    EmptySpace::enumed(
        parse_name(obj, "EmptySpace"),
    )
}

fn parse_horizontal_splitter(obj: &Object) -> Option<Components> {
    let name = match obj.get("name") {
        Some(name) => match name.as_str() {
            Some(name) => name,
            _ => "HorizontalSplitter",
        },
        None => "HorizontalSplitter",
    };

    let borders = match obj.get("borders") {
        Some(val) => match val.as_str() {
            Some("false") => false,
            _ => true,
        },
        _ => true,
    };

    let children = match obj.get("children") {
        Some(val) => match val {
            JsonValue::Array(arr) => {
                let mut children = Vec::new();

                for val in arr {
                    if let Some(c) = parse_component(val) {
                        if let Some(size) = parse_size(val) {
                            children.push(Panel::new(size, c));
                        }
                    }
                }

                children
            },
            _ => Vec::new(),
        },
        None => Vec::new(),
    };

    Some(HorizontalSplitter::enumed(name, borders, children))
}

fn parse_vertical_splitter(obj: &Object) -> Option<Components> {
    let name = match obj.get("name") {
        Some(name) => match name.as_str() {
            Some(name) => name,
            _ => "VerticalSplitter",
        },
        None => "VerticalSplitter",
    };

    let borders = match obj.get("borders") {
        Some(val) => match val.as_str() {
            Some("false") => false,
            _ => true,
        },
        _ => true,
    };

    let children = match obj.get("children") {
        Some(val) => match val {
            JsonValue::Array(arr) => {
                let mut children = Vec::new();

                for val in arr {
                    if let Some(c) = parse_component(val) {
                        if let Some(size) = parse_size(val) {
                            children.push(Panel::new(size, c));
                        }
                    }
                }

                children
            },
            _ => Vec::new(),
        },
        None => Vec::new(),
    };

    Some(VerticalSplitter::enumed(name, borders, children))
}

fn parse_size(val: &JsonValue) -> Option<Size> {
    if let JsonValue::Object(obj) = val {
        match obj.get("size") {
            Some(val) => match val {
                JsonValue::String(s) => {
                    if s.ends_with("%") {
                        let mut s = s.to_string();
                        s.pop();

                        if let Ok(val) = s.parse::<u8>() {
                            Some(Size::Percent(val))
                        } else {
                            None
                        }
                    } else {
                        if let Ok(val) = s.parse::<u16>() {
                            Some(Size::Absolute(val))
                        } else {
                            Some(Size::Remainder)
                        }
                    }
                },
                JsonValue::Short(s) => {
                    let s = s.as_str();

                    if s.ends_with("%") {
                        let mut s = s.to_string();
                        s.pop();

                        if let Ok(val) = s.parse::<u8>() {
                            Some(Size::Percent(val))
                        } else {
                            None
                        }
                    } else {
                        if let Ok(val) = s.parse::<u16>() {
                            Some(Size::Absolute(val))
                        } else {
                            Some(Size::Remainder)
                        }
                    }
                }
                _ => {
                    eprintln!("Error: parse_size: size is not string");
                    None
                },
            },
            None => Some(Size::Remainder),
        }
    } else {
        eprintln!("Error: parse_size: jsonvalue is not an object");
        None
    }
}

fn parse_color(obj: &Object, key: &str) -> Color {
    match obj.get(key) {
        Some(JsonValue::Short(s)) => match s.as_str() {
            "Black" => Color::Black,
            "Red" => Color::Red,
            "Green" => Color::Green,
            "Yellow" => Color::Yellow,
            "Blue" => Color::Blue,
            "Magenta" => Color::Magenta,
            "Cyan" => Color::Cyan,
            "White" => Color::White,
            "BrightBlack" => Color::BrightBlack,
            "BrightRed" => Color::BrightRed,
            "BrightGreen" => Color::BrightGreen,
            "BrightYellow" => Color::BrightYellow,
            "BrightBlue" => Color::BrightBlue,
            "BrightMagenta" => Color::BrightMagenta,
            "BrightCyan" => Color::BrightCyan,
            "BrightWhite" => Color::BrightWhite,
            "Reset" => Color::Reset,
            bad => {
                eprintln!("Error: parse_color: invalid color {:?}. Defaulting to Color::Reset", bad);
                Color::Reset
            },
        },
        Some(JsonValue::String(s)) => match s.as_str() {
            "Black" => Color::Black,
            "Red" => Color::Red,
            "Green" => Color::Green,
            "Yellow" => Color::Yellow,
            "Blue" => Color::Blue,
            "Magenta" => Color::Magenta,
            "Cyan" => Color::Cyan,
            "White" => Color::White,
            "BrightBlack" => Color::BrightBlack,
            "BrightRed" => Color::BrightRed,
            "BrightGreen" => Color::BrightGreen,
            "BrightYellow" => Color::BrightYellow,
            "BrightBlue" => Color::BrightBlue,
            "BrightMagenta" => Color::BrightMagenta,
            "BrightCyan" => Color::BrightCyan,
            "BrightWhite" => Color::BrightWhite,
            "Reset" => Color::Reset,
            bad => {
                eprintln!("Error: parse_color: invalid color {:?}. Defaulting to Color::Reset", bad);
                Color::Reset
            },
        },
        Some(JsonValue::Object(obj)) => match parse_color_rgb(obj) {
            Some(color) => color,
            None => {
                eprintln!("Error: parse_color: bad rgb color '{:?}'. Defaulting to Color::Reset", obj);
                Color::Reset
            },
        }
        _ => Color::Reset,
    }
}

fn parse_color_rgb(obj: &Object) -> Option<Color> {
    let r = match parse_color_rgb_part(obj, "r") {
        Some(num) => num,
        None => return None,
    };

    let g = match parse_color_rgb_part(obj, "g") {
        Some(num) => num,
        None => return None,
    };

    let b = match parse_color_rgb_part(obj, "b") {
        Some(num) => num,
        None => return None,
    };

    Some(Color::RGB(r, g, b))
}

fn parse_color_rgb_part(obj: &Object, key: &str) -> Option<u8> {
    let ret = match obj.get(key) {
        Some(JsonValue::Number(num)) => num.as_parts().1,
        _ => return None,
    };

    if ret > 5 {
        Some(5)
    } else {
        Some(ret as u8)
    }
}

fn parse_alignment(obj: &Object) -> Alignment {
    match obj.get("alignment") {
        Some(JsonValue::String(s)) => match s.as_str() {
            "Center" => Alignment::Center,
            "Right" => Alignment::Right,
            "Left" => Alignment::Left,
            _ => {
                eprintln!("Error: parse_alignment: invalid value for text_align. Defaulting to Alignment::Left");
                Alignment::Left
            },
        },
        Some(JsonValue::Short(s)) => match s.as_str() {
            "Center" => Alignment::Center,
            "Right" => Alignment::Right,
            "Left" => Alignment::Left,
            _ => {
                eprintln!("Error: parse_alignment: invalid value for text_align. Defaulting to Alignment::Left");
                Alignment::Left
            },
        },
        Some(_) => {
            eprintln!("Error: parse_alignment: text_align is not a string");
            Alignment::Left
        },
        None => Alignment::Left,
    }
}
