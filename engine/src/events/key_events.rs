use std::fmt;
use std::fmt::Debug;

use crate::events::Event;
use crate::events::EventCategory;
use crate::events::EventType;

pub trait KeyEvent {
    const KEY_EVENT_CATEGORY: EventCategory = EventCategory::from_bits_retain(
        EventCategory::Keyboard.bits() | EventCategory::Input.bits(),
    );
    fn get_keycode(&self) -> usize;
}

pub struct KeyPressedEvent {
    handled: bool,
    keycode: usize,
    repeat_count: u16,
}

impl KeyPressedEvent {
    pub fn new(keycode: usize, repeat_count: u16) -> Self {
        Self {
            keycode,
            repeat_count,
            handled: false,
        }
    }
    pub const fn get_repeat_count(&self) -> u16 {
        self.repeat_count
    }
}

impl KeyEvent for KeyPressedEvent {
    fn get_keycode(&self) -> usize {
        self.keycode
    }
}

impl Event for KeyPressedEvent {
    const EVENT_TYPE: EventType = EventType::KeyPressed;
    const EVENT_CATEGORY: EventCategory = Self::KEY_EVENT_CATEGORY;
}

impl Debug for KeyPressedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}Event: {} ({} times)",
            Self::EVENT_TYPE,
            self.keycode,
            self.repeat_count
        )
    }
}

pub struct KeyReleasedEvent {
    keycode: usize,
}

impl KeyReleasedEvent {
    pub fn new(keycode: usize) -> Self {
        Self { keycode }
    }
}

impl KeyEvent for KeyReleasedEvent {
    fn get_keycode(&self) -> usize {
        self.keycode
    }
}

impl Event for KeyReleasedEvent {
    const EVENT_TYPE: EventType = EventType::KeyReleased;
    const EVENT_CATEGORY: EventCategory = Self::KEY_EVENT_CATEGORY;
}

impl Debug for KeyReleasedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}Event: {}", Self::EVENT_TYPE, self.keycode)
    }
}
