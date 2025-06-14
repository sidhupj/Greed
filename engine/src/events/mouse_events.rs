use std::fmt::{self, Debug};

use crate::events::{Event, EventCategory, EventType};

pub trait MouseEvent {
    const MOUSE_EVENT_CATEGORY: EventCategory =
        EventCategory::from_bits_retain(EventCategory::Input.bits() | EventCategory::Mouse.bits());
}
pub struct MouseMovedEvent {
    handled: bool,
    x: f32,
    y: f32,
}

impl MouseMovedEvent {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            handled: false,
            x,
            y,
        }
    }
    pub const fn get_x(&self) -> f32 {
        self.x
    }
    pub const fn get_y(&self) -> f32 {
        self.y
    }
}

impl Event for MouseMovedEvent {
    const EVENT_TYPE: EventType = EventType::MouseMoved;
    const EVENT_CATEGORY: EventCategory = Self::MOUSE_EVENT_CATEGORY;
}

impl Debug for MouseMovedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}Event: {} {}", Self::EVENT_TYPE, self.x, self.y)
    }
}

impl MouseEvent for MouseMovedEvent {}

pub struct MouseScrolledEvent {
    handled: bool,
    x_offset: f32,
    y_offset: f32,
}

impl MouseScrolledEvent {
    pub fn new(x_offset: f32, y_offset: f32) -> Self {
        Self {
            handled: false,
            x_offset,
            y_offset,
        }
    }
    pub const fn get_x_offset(&self) -> f32 {
        self.x_offset
    }
    pub const fn get_y_offset(&self) -> f32 {
        self.y_offset
    }
}

impl Event for MouseScrolledEvent {
    const EVENT_TYPE: EventType = EventType::MouseScrolled;
    const EVENT_CATEGORY: EventCategory =
        EventCategory::from_bits_retain(EventCategory::Input.bits() | EventCategory::Mouse.bits());
}

impl Debug for MouseScrolledEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}Event: {} {}",
            Self::EVENT_TYPE,
            self.x_offset,
            self.y_offset
        )
    }
}
pub trait MouseButtonEvent {
    const MOUSE_BUTTON_EVENT_CATEGORY: EventCategory = EventCategory::from_bits_retain(
        EventCategory::Input.bits()
            | EventCategory::Mouse.bits()
            | EventCategory::MouseButton.bits(),
    );
    fn get_mouse_button(&self) -> usize;
}

pub struct MouseButtonPressedEvent {
    handled: bool,
    button: usize,
}

impl MouseButtonPressedEvent {
    pub fn new(button: usize) -> Self {
        Self {
            handled: false,
            button,
        }
    }
}

impl MouseButtonEvent for MouseButtonPressedEvent {
    fn get_mouse_button(&self) -> usize {
        self.button
    }
}

impl Event for MouseButtonPressedEvent {
    const EVENT_TYPE: EventType = EventType::MouseButtonPressed;
    const EVENT_CATEGORY: EventCategory = Self::MOUSE_BUTTON_EVENT_CATEGORY;
}

impl Debug for MouseButtonPressedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}Event: {}", Self::EVENT_TYPE, self.button)
    }
}

pub struct MouseButtonReleasedEvent {
    button: usize,
    handled: bool,
}

impl MouseButtonReleasedEvent {
    pub fn new(button: usize) -> Self {
        Self {
            button,
            handled: false,
        }
    }
}

impl MouseButtonEvent for MouseButtonReleasedEvent {
    fn get_mouse_button(&self) -> usize {
        self.button
    }
}

impl Event for MouseButtonReleasedEvent {
    const EVENT_TYPE: EventType = EventType::MouseButtonReleased;
    const EVENT_CATEGORY: EventCategory = Self::MOUSE_BUTTON_EVENT_CATEGORY;
}

impl Debug for MouseButtonReleasedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}Event: {}", Self::EVENT_TYPE, self.button)
    }
}
