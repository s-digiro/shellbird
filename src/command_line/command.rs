use crate::event::*;

pub fn parse(cmd: &Vec<&str>) -> Option<Event> {
    match get_lowercase(cmd, 0) {
        Some(s) => match s.as_str() {
            "invalid" => match cmd.get(1) {
                Some(s) => Some(Event::ToApp(AppEvent::InvalidCommand(s.to_string()))),
                None => Some(Event::ToApp(AppEvent::InvalidCommand(String::new()))),
            },

            "echo"
            | "respond"
            | "response"
            | "commandresponse"
            | "commandrespond" => match cmd.get(1) {
                Some(s) => Some(Event::ToApp(AppEvent::CommandResponse(s.to_string()))),
                None => Some(Event::ToApp(AppEvent::CommandResponse(String::new()))),
            },

            "quit"
            | "q"
            | "exit" => Some(Event::ToApp(AppEvent::Quit)),

            "switchscreen"
            | "screen" => match get_usize(cmd, 1) {
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
            | "s" => match get_lowercase(cmd, 1) {
                Some(s) => Some(Event::ToFocus(FocusEvent::Search(s.to_string()))),
                None => None,
            },

            "goto"
            | "go"
            | "g"
            | "to" => match get_usize(cmd, 1) {
                Some(num) => Some(Event::ToFocus(FocusEvent::GoTo(num))),
                None => None,
            }

            "togglepause"
            | "pause"
            | "toggle" => Some(Event::ToMpd(MpdEvent::TogglePause)),

            "clear"
            | "clearqueue" => Some(Event::ToMpd(MpdEvent::ClearQueue)),

            "bind"
            | "bindkey" => match cmd.get(1) {
                Some(s) => {
                    let new_cmd = cmd.iter()
                        .skip(2)
                        .map(|s| *s)
                        .collect();

                    match parse(&new_cmd) {
                        Some(e) => match BindableEvent::from_event(e) {
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

fn get_lowercase(cmd: &Vec<&str>, i: usize) -> Option<String> {
    match cmd.get(i) {
        Some(s) => Some(s.to_string().to_lowercase()),
        None => None,
    }
}

fn get_usize(cmd: &Vec<&str>, i: usize) -> Option<usize> {
    match cmd.get(i) {
        Some(s) => match s.parse::<usize>() {
            Ok(num) => Some(num),
            _ => None,
        },
        None => None,
    }
}
