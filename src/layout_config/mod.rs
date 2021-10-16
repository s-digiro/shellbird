use json::JsonValue;
use json::object::Object;

use std::collections::HashMap;
use std::fs;
use std::error::Error;

use crate::components::*;
use crate::screen::Screen;

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
                    "EmptySpace" => parse_empty_space(obj),
                    "VerticalSplitter" => parse_vertical_splitter(obj),
                    "PlaceHolder" => parse_place_holder(obj),
                    "TagDisplay" => parse_tag_display(obj),
                    "TitleDisplay" => parse_title_display(obj),
                    "Queue" => parse_queue(obj),
                    "PlaylistMenu" => parse_playlist_menu(obj),
                    "TrackMenu" => parse_track_menu(obj),
                    "TagMenu" => parse_tag_menu(obj),
                    "StyleMenu" => parse_style_menu(obj),
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

fn parse_style_menu(obj: &Object) -> Option<Components> {
    let name = parse_name(obj, "StyleMenu");
    let parent = parse_parent(obj);

    Some(StyleMenu::enumed(name, parent))
}

fn parse_tag_menu(obj: &Object) -> Option<Components> {
    let name = parse_name(obj, "TagMenu");
    let parent = parse_parent(obj);
    let tag = parse_tag(obj, "Artist");

    Some(TagMenu::enumed(name, tag, parent))
}

fn parse_track_menu(obj: &Object) -> Option<Components> {
    let name = parse_name(obj, "TrackMenu");
    let parent = parse_parent(obj);

    Some(TrackMenu::enumed(name, parent))
}

fn parse_playlist_menu(obj: &Object) -> Option<Components> {
    let name = parse_name(obj, "PlaylistMenu");

    Some(PlaylistMenu::enumed(name))
}

fn parse_queue(obj: &Object) -> Option<Components> {
    let name = parse_name(obj, "Queue");

    Some(Queue::enumed(name))
}

fn parse_title_display(obj: &Object) -> Option<Components> {
    let name = parse_name(obj, "TitleDisplay");

    Some(TitleDisplay::enumed(name))
}

fn parse_tag_display(obj: &Object) -> Option<Components> {
    let name = parse_name(obj, "TagDisplay");

    let tag = parse_tag(obj, "Artist");

    Some(TagDisplay::enumed(name, tag))
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

fn parse_place_holder(obj: &Object) -> Option<Components> {
    let name = match obj.get("name") {
        Some(name) => match name.as_str() {
            Some(name) => name,
            _ => "PlaceHolder",
        },
        _ => "PlaceHolder",
    };

    Some(PlaceHolder::enumed(name))
}

fn parse_empty_space(obj: &Object) -> Option<Components> {
    let name = match obj.get("name") {
        Some(name) => match name.as_str() {
            Some(name) => name,
            _ => "EmptySpace",
        },
        _ => "EmptySpace",
    };

    Some(EmptySpace::enumed(name))
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
            _ => false,
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
            _ => false,
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
                            None
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
                            None
                        }
                    }
                }
                _ => {
                    eprintln!("Error: parse_size: size is not string");
                    None
                },
            },
            None => {
                eprintln!("Error: parse_size: no size key");
                None
            }
        }
    } else {
        eprintln!("Error: parse_size: jsonvalue is not an object");
        None
    }
}
