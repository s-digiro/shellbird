use crate::event::*;

pub fn replace_macros(cmd: Vec<&str>) -> Vec<String> {
    let mut ret: Vec<String> = cmd.clone().iter()
        .map(|s| s.to_string()).collect();

    for (i, token) in cmd.iter().enumerate() {
        match *token {
            "<space>" => ret[i] = String::from(" "),
            _ => (),
        }
    }

    ret
}

pub fn parse(cmd: &Vec<&str>) -> Option<Event> {
    let cmd = replace_macros(cmd.clone());

    match get_lowercase(&cmd, 0) {
        Some(s) => match s.as_str() {
            "echo" => match cmd.get(1) {
                Some(s) => Some(Event::ToCommandLine(CommandLineEvent::Echo(s.to_string()))),
                None => None,
            },

            "quit"
            | "q"
            | "exit" => Some(Event::ToApp(AppEvent::Quit)),

            "switchscreen"
            | "screen" => match get_usize(&cmd, 1) {
                Some(num) => Some(Event::ToApp(AppEvent::SwitchScreen(num))),
                None => None,
            },

            "focusnext" => Some(Event::ToScreen(ScreenEvent::FocusNext)),
            "focusprev" => Some(Event::ToScreen(ScreenEvent::FocusPrev)),
            "next" => Some(Event::ToFocus(FocusEvent::Next)),
            "prev" => Some(Event::ToFocus(FocusEvent::Prev)),
            "select" => Some(Event::ToFocus(FocusEvent::Select)),

            "top"
            | "gotop"
            | "gototop"
            | "totop" => Some(Event::ToFocus(FocusEvent::GoToTop)),

            "bottom"
            | "gobottom"
            | "gotobottom"
            | "tobottom"
            | "bot"
            | "gobot"
            | "gotobot"
            | "tobot" => Some(Event::ToFocus(FocusEvent::GoToBottom)),

            "search"
            | "s" => match get_lowercase(&cmd, 1) {
                Some(s) => Some(Event::ToFocus(FocusEvent::Search(s.to_string()))),
                None => None,
            },

            "goto"
            | "go"
            | "g"
            | "to" => match get_usize(&cmd, 1) {
                Some(num) => Some(Event::ToFocus(FocusEvent::GoTo(num))),
                None => None,
            }

            "togglepause"
            | "pause"
            | "toggle" => Some(Event::ToMpd(MpdEvent::TogglePause)),

            "clear"
            | "clearqueue" => Some(Event::ToMpd(MpdEvent::ClearQueue)),

            "random" => Some(Event::ToMpd(MpdEvent::Random)),

            "bind"
            | "bindkey" => match cmd.get(1) {
                Some(s) => {
                    let new_cmd = cmd.iter()
                        .skip(2)
                        .map(|s| s.as_str())
                        .collect();

                    match parse(&new_cmd) {
                        Some(e) => match NestableEvent::from_event(e) {
                            Some(e) => Some(Event::BindKey(s.to_string(), e)),
                            None => None,
                        },
                        _ => None,
                    }
                },
                None => None,
            },

            _ => None,
        }
        None => None,
    }
}

fn get_lowercase(cmd: &Vec<String>, i: usize) -> Option<String> {
    match cmd.get(i) {
        Some(s) => Some(s.to_string().to_lowercase()),
        None => None,
    }
}

fn get_usize(cmd: &Vec<String>, i: usize) -> Option<usize> {
    match cmd.get(i) {
        Some(s) => match s.parse::<usize>() {
            Ok(num) => Some(num),
            _ => None,
        },
        None => None,
    }
}
