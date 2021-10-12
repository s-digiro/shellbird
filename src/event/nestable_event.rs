use super::*;

#[derive(Debug)]
#[derive(Clone)]
pub enum NestableEvent {
    ToApp(AppEvent),
    ToScreen(ScreenEvent),
    ToGlobal(GlobalEvent),
    ToFocus(FocusEvent),
    ToMpd(MpdEvent),
    ToCommandLine(CommandLineEvent),
}

impl NestableEvent {
    pub fn from_event(e: Event) -> Option<NestableEvent> {
        match e {
            Event::ToApp(e) => Some(NestableEvent::ToApp(e)),
            Event::ToScreen(e) => Some(NestableEvent::ToScreen(e)),
            Event::ToGlobal(e) => Some(NestableEvent::ToGlobal(e)),
            Event::ToFocus(e) => Some(NestableEvent::ToFocus(e)),
            Event::ToMpd(e) => Some(NestableEvent::ToMpd(e)),
            Event::ToCommandLine(e) => Some(NestableEvent::ToCommandLine(e)),
            _ => None,
        }
    }

    pub fn to_event(self) -> Event {
        match self {
            NestableEvent::ToApp(e) => Event::ToApp(e),
            NestableEvent::ToScreen(e) => Event::ToScreen(e),
            NestableEvent::ToGlobal(e) => Event::ToGlobal(e),
            NestableEvent::ToFocus(e) => Event::ToFocus(e),
            NestableEvent::ToMpd(e) => Event::ToMpd(e),
            NestableEvent::ToCommandLine(e) => Event::ToCommandLine(e),
        }
    }

}
