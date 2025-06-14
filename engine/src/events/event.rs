use bitflags::bitflags;

#[derive(Debug, Eq, PartialEq)]
pub enum EventType {
    None,
    WindowClose,
    WindowResize,
    WindowFocus,
    WindowLostFocus,
    WindowMoved,
    AppTick,
    AppUpdate,
    AppRender,
    KeyPressed,
    KeyReleased,
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseScrolled,
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct EventCategory: u8 {
        const None = 0;
        const Application = 1 << 0;
        const Input = 1 << 1;
        const Keyboard = 1 << 2;
        const Mouse = 1 << 3;
        const MouseButton = 1 << 4;
    }
}

pub trait Event {
    const EVENT_TYPE: EventType;
    const EVENT_CATEGORY: EventCategory;

    fn get_event_type(&self) -> EventType {
        Self::EVENT_TYPE
    }
    // fn get_name(&self) -> &'static str;
    // fn get_category_flags(&self) -> usize;
    fn is_in_category(&self, category: EventCategory) -> bool {
        return (Self::EVENT_CATEGORY & category).bits() != 0;
    }
}

pub struct EventDispatcher<'a, T>
where
    T: Event,
{
    event: &'a T,
}

impl<'a, T> EventDispatcher<'a, T>
where
    T: Event,
{
    pub fn new(event: &'a T) -> Self {
        Self { event }
    }

    pub fn dispatch(&self, func: fn(&'a T) -> bool) -> bool {
        func(self.event)
    }
}
