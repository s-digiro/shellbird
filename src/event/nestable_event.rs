use super::*;

#[derive(Debug)]
#[derive(Clone)]
pub enum NestableEvent {
    ToApp(AppEvent),
    ToScreen(ScreenEvent),
    ToAllComponents(ComponentEvent),
    ToFocus(ComponentEvent),
    ToMpd(MpdEvent),
    ToCommandLine(CommandLineEvent),
    ToComponent(String, ComponentEvent),
}

impl NestableEvent {
    pub fn from_event(e: Event) -> Option<NestableEvent> {
        match e {
            Event::ToApp(e) => Some(NestableEvent::ToApp(e)),
            Event::ToScreen(e) => Some(NestableEvent::ToScreen(e)),
            Event::ToAllComponents(e) => Some(NestableEvent::ToAllComponents(e)),
            Event::ToFocus(e) => Some(NestableEvent::ToFocus(e)),
            Event::ToMpd(e) => Some(NestableEvent::ToMpd(e)),
            Event::ToCommandLine(e) => Some(NestableEvent::ToCommandLine(e)),
            Event::ToComponent(s, e) => Some(NestableEvent::ToComponent(s, e)),
            _ => None,
        }
    }

    pub fn to_event(self) -> Event {
        match self {
            NestableEvent::ToApp(e) => Event::ToApp(e),
            NestableEvent::ToScreen(e) => Event::ToScreen(e),
            NestableEvent::ToAllComponents(e) => Event::ToAllComponents(e),
            NestableEvent::ToFocus(e) => Event::ToFocus(e),
            NestableEvent::ToMpd(e) => Event::ToMpd(e),
            NestableEvent::ToCommandLine(e) => Event::ToCommandLine(e),
            NestableEvent::ToComponent(s, e) => Event::ToComponent(s, e),
        }
    }

}
