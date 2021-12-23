/* Functionality for parsing layout file
   Copyright (C) 2020-2021 Sean DiGirolamo

This file is part of Shellbird.

Shellbird is free software; you can redistribute it and/or modify it
under the terms of the GNU General Public License as published by the
Free Software Foundation; either version 3, or (at your option) any
later version.

Shellbird is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License
along with Shellbird; see the file COPYING.  If not see
<http://www.gnu.org/licenses/>.  */

use json::JsonValue;
use json::object::Object;

use unicode_truncate::Alignment;

use std::collections::HashMap;
use std::fs;
use std::error::Error;

use crate::components::*;
use crate::color::Color;

#[cfg(test)]
mod tests;

pub fn load(path: &str) -> Result<HashMap<String, Components>, Box<dyn Error>> {
    let file_contents = match fs::read_to_string(path) {
        Ok(fc) => fc,
        Err(e) => return Err(Box::new(e)),
    };

    let val = match json::parse(&file_contents) {
        Ok(x) => x,
        Err(e) => return Err(Box::new(e)),
    };

    let mut ret = HashMap::new();

    if let JsonValue::Array(arr) = val {
        for val in arr {
            if let JsonValue::Object(obj) = val {
                let (name, c) = parse_component(&obj, &mut ret);

                if let Some(c) = c {
                    ret.insert(name, c);
                }
            } else {
                eprintln!(
                    "Warning: layout_config::load: Base Component is not an \
                        object. It is being skipped."
                );
            }
        }
    }

    Ok(ret)
}

fn parse_component(
    obj: &Object,
    map: &mut HashMap<String, Components>
) -> (String, Option<Components>) {
    if let Some(component) = obj.get("component") {
        let c = match component.as_str() {
            Some("HorizontalSplitter") => parse_horizontal_splitter(obj, map),
            Some("EmptySpace") => parse_empty_space(obj),
            Some("VerticalSplitter") => parse_vertical_splitter(obj, map),
            Some("PlaceHolder") => parse_place_holder(obj),
            Some("TagDisplay") => parse_tag_display(obj),
            Some("TitleDisplay") => parse_title_display(obj),
            Some("Queue") => parse_queue(obj),
            Some("PlaylistMenu") => parse_playlist_menu(obj),
            Some("TrackMenu") => parse_track_menu(obj),
            Some("TagMenu") => parse_tag_menu(obj),
            Some("StyleMenu") => parse_style_menu(obj),
            _ => {
                eprintln!(
                    "Error: parse_component: component with component has \
                        invalid value for component. Creating ErrorBox."
                );
                ErrorBox::enumed()
            },
        };

        (c.name().to_string(), Some(c))
    } else if let Some(name) = obj.get("name") {
        if let Some(name) = name.as_str() {
            (name.to_string(), None)
        } else {
            eprintln!(
                "Error: parse_component: component with no component but with \
                    name has name that is not a string. Creating ErrorBox \
                    instead."
            );
            let error = ErrorBox::enumed();
            (error.name().to_string(), Some(error))
        }
    } else {
        eprintln!(
            "Error: parse_component: component object has no component field \
            or name field. Need at least one. Creating ErrorBox."
        );
        let error = ErrorBox::enumed();
        (error.name().to_string(), Some(error))
    }
}

fn parse_style_menu(obj: &Object) -> Components {
    StyleMenu::enumed(
        parse_string(obj, "name").unwrap_or("StyleMenu"),
        parse_color(obj, "color"),
        parse_color(obj, "focus_color"),
        parse_optional_string(obj, "title"),
        parse_alignment(obj, "title_alignment"),
        parse_alignment(obj, "menu_alignment"),
        parse_optional_string(obj, "parent")
    )
}

fn parse_tag_menu(obj: &Object) -> Components {
    TagMenu::enumed(
        parse_string(obj, "name").unwrap_or("TagMenu"),
        parse_color(obj, "color"),
        parse_color(obj, "focus_color"),
        parse_optional_string(obj, "title"),
        parse_alignment(obj, "title_alignment"),
        parse_alignment(obj, "menu_alignment"),
        parse_string(obj, "tag").unwrap_or("Artist"),
        parse_optional_string(obj, "multitag_separator"),
        parse_optional_string(obj, "parent"),
    )
}

fn parse_track_menu(obj: &Object) -> Components {
    TrackMenu::enumed(
        parse_string(obj, "name").unwrap_or("TrackMenu"),
        parse_color(obj, "color"),
        parse_color(obj, "focus_color"),
        parse_optional_string(obj, "title"),
        parse_alignment(obj, "title_alignment"),
        parse_alignment(obj, "menu_alignment"),
        parse_optional_string(obj, "parent"),
    )
}

fn parse_playlist_menu(obj: &Object) -> Components {
    PlaylistMenu::enumed(
        parse_string(obj, "name").unwrap_or("PlaylistMenu"),
        parse_color(obj, "color"),
        parse_color(obj, "focus_color"),
        parse_optional_string(obj, "title"),
        parse_alignment(obj, "title_alignment"),
        parse_alignment(obj, "menu_alignment"),
    )
}

fn parse_queue(obj: &Object) -> Components {
    Queue::enumed(
        parse_string(obj, "name").unwrap_or("Queue"),
        parse_color(obj, "color"),
        parse_color(obj, "focus_color"),
        parse_optional_string(obj, "title"),
        parse_alignment(obj, "title_alignment"),
        parse_alignment(obj, "menu_alignment"),
    )
}

fn parse_title_display(obj: &Object) -> Components {
    TitleDisplay::enumed(
        parse_string(obj, "name").unwrap_or("TitleDisplay"),
        parse_color(obj, "color"),
        parse_alignment(obj, "alignment"),
    )
}

fn parse_tag_display(obj: &Object) -> Components {
    TagDisplay::enumed(
        parse_string(obj, "name").unwrap_or( "TagDisplay"),
        parse_color(obj, "color"),
        parse_alignment(obj, "alignment"),
        parse_string(obj, "tag").unwrap_or("Artist"),
    )
}

fn parse_string<'a>(obj: &'a Object, key: &str) -> Option<&'a str> {
    match obj.get(key) {
        Some(s) => s.as_str(),
        None => None,
    }
}

fn parse_optional_string<'a>(obj: &'a Object, key: &str) -> Option<String> {
    match obj.get(key) {
        Some(s) => Some(s.to_string()),
        None => None,
    }
}

fn parse_place_holder(obj: &Object) -> Components {
    PlaceHolder::enumed(
        parse_string(obj, "name").unwrap_or("PlaceHolder"),
        parse_color(obj, "color"),
    )
}

fn parse_empty_space(obj: &Object) -> Components {
    EmptySpace::enumed(
        parse_string(obj, "name").unwrap_or("EmptySpace"),
    )
}

fn parse_vector_splitter_children(
    obj: &Object,
    map: &mut HashMap<String, Components>,
) -> Vec<Panel> {
    match obj.get("children") {
        Some(val) => match val {
            JsonValue::Array(arr) => {
                let mut children = Vec::new();

                for val in arr {
                    match val {
                        JsonValue::Object(obj) => {
                            let (name, c) = parse_component(obj, map);
                            let size = parse_size(obj);

                            if let Some(c) = c {
                                map.insert(name.clone(), c);
                            }

                            let panel = Panel::new(size, name);
                            children.push(panel);
                        },
                        _ => eprintln!(
                            "Error: parse_vector_splitter_children: child is \
                                not an object"
                        ),
                    }
                }

                children
            },
            _ => {
                eprintln!(
                    "Error: parse_vector_splitter_children: child is not an \
                        array. Initializing splitter with no children"
                );
                Vec::new()
            },
        },
        None => Vec::new(),
    }
}

fn parse_horizontal_splitter(
    obj: &Object,
    map: &mut HashMap<String, Components>
) -> Components {
    HorizontalSplitter::enumed(
        parse_string(obj, "name").unwrap_or("HorizontalSplitter"),
        parse_bool(obj, "borders").unwrap_or(true),
        parse_vector_splitter_children(obj, map),
    )
}

fn parse_bool(obj: &Object, key: &str) -> Option<bool> {
    match obj.get(key) {
        Some(val) => val.as_bool(),
        None => None,
    }
}

fn parse_vertical_splitter(
    obj: &Object,
    map: &mut HashMap<String, Components>,
) -> Components {
    VerticalSplitter::enumed(
        parse_string(obj, "name").unwrap_or("VerticalSplitter"),
        parse_bool(obj, "borders").unwrap_or(true),
        parse_vector_splitter_children(obj, map),
    )
}

fn parse_size(obj: &Object) -> Size {
    match obj.get("size") {
        Some(val) => match val.as_str() {
            Some(s) => {
                if s.ends_with("%") {
                    let mut s = s.to_string();
                    s.pop();

                    if let Ok(val) = s.parse::<u8>() {
                        Size::Percent(val)
                    } else {
                        eprintln!("Error: parse_size: String cannot be parsed into u8. Defaulting to Remainder.");
                        Size::Remainder
                    }
                } else if s == "Remainder" {
                    Size::Remainder
                } else {
                    if let Ok(val) = s.parse::<u16>() {
                        Size::Absolute(val)
                    } else {
                        eprintln!("Error: parse_size: String cannot be parsed into u16. Defaulting to Remainder");
                        Size::Remainder
                    }
                }
            },
            _ => {
                eprintln!("Error: parse_size: Size cannot be parsed into string. Defaulting to Remainder");
                Size::Remainder
            },
        },
        None => Size::Remainder,
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

fn parse_alignment(obj: &Object, key: &str) -> Alignment {
    match obj.get(key) {
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
