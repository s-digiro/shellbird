use super::*;

#[derive(Debug)]
#[derive(Clone)]
pub enum BindableEvent {
    ToApp(AppEvent),
    ToScreen(ScreenEvent),
    ToGlobal(GlobalEvent),
    ToFocus(FocusEvent),
    ToMpd(MpdEvent),
}

impl BindableEvent {
    pub fn from_event(e: Event) -> Option<BindableEvent> {
        match e {
            Event::ToApp(e) => Some(BindableEvent::ToApp(e)),
            Event::ToScreen(e) => Some(BindableEvent::ToScreen(e)),
            Event::ToGlobal(e) => Some(BindableEvent::ToGlobal(e)),
            Event::ToFocus(e) => Some(BindableEvent::ToFocus(e)),
            Event::ToMpd(e) => Some(BindableEvent::ToMpd(e)),
            _ => None,
        }
    }

    pub fn to_event(self) -> Event {
        match self {
            BindableEvent::ToApp(e) => Event::ToApp(e),
            BindableEvent::ToScreen(e) => Event::ToScreen(e),
            BindableEvent::ToGlobal(e) => Event::ToGlobal(e),
            BindableEvent::ToFocus(e) => Event::ToFocus(e),
            BindableEvent::ToMpd(e) => Event::ToMpd(e),
        }
    }

}
