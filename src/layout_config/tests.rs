use super::*;
use json::object;

#[test]
fn test_parse_size_absolute() -> Result<(), String> {
    let input = object! {
        "size": "10".to_string(),
    };

    let target = Size::Absolute(10);

    assert_eq!(parse_size(&input), Some(target));

    Ok(())
}

#[test]
fn test_parse_size_percent() -> Result<(), String> {
    let input = object! {
        "size": "10%",
    };

    let target = Size::Percent(10);

    assert_eq!(parse_size(&input), Some(target));

    Ok(())
}

#[test]
fn test_parse_size_number() -> Result<(), String> {
    let input = object! {
        "size": 10,
    };

    assert_eq!(parse_size(&input), None);

    Ok(())
}

#[test]
fn test_parse_style_menu() -> Result<(), String> {
    let target = Some(StyleMenu::enumed("a name", None));

    let input = object! {
        "component": "StyleMenu",
        "name": "a name",
        "size": "20%",
    };

    assert_eq!(target, parse_style_menu(as_object(&input)));

    Ok(())
}

#[test]
fn test_parse_horizontal_splitter() -> Result<(), String> {
    let target = Some(HorizontalSplitter::enumed(
        "a name",
        true,
        vec![
            Panel::new(
                Size::Percent(10),
                PlaceHolder::enumed("a name 1"),
            ),
            Panel::new(
                Size::Percent(3),
                TagDisplay::enumed("a name 2", "Genre"),
            ),
        ],
    ));

    let input = object! {
        "component": "HorizontalSplitter",
        "name": "a name",
        "border": "true",
        "children": [
            {
                "component": "PlaceHolder",
                "name": "a name 1",
                "size": "10%",
            },
            {
                "component": "TagDisplay",
                "name": "a name 2",
                "size": "3%",
                "tag": "Genre",
            },
        ],
    };

    assert_eq!(target, parse_horizontal_splitter(as_object(&input)));

    Ok(())
}

fn as_object(val: &JsonValue) -> &Object {
    match val {
        JsonValue::Object(obj) => obj,
        _ => panic!("Test failed not an object"),
    }
}
