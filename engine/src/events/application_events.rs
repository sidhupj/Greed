use std::fmt::Debug;

use crate::events::{Event, EventCategory, EventType};

pub trait WindowEvent {
    const WINDOW_EVENT_CATEGORY: EventCategory = EventCategory::Application;
}

#[derive(Debug)]
pub struct WindowCloseEvent;

impl WindowCloseEvent {
    pub fn new() -> Self {
        Self
    }
}

impl WindowEvent for WindowCloseEvent {}

impl Event for WindowCloseEvent {
    const EVENT_TYPE: EventType = EventType::WindowClose;
    const EVENT_CATEGORY: EventCategory = Self::WINDOW_EVENT_CATEGORY;
}

pub struct WindowResizeEvent {
    width: u32,
    height: u32,
}

impl WindowResizeEvent {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    pub const fn get_width(&self) -> u32 {
        self.width
    }
    pub const fn get_height(&self) -> u32 {
        self.height
    }
}

impl WindowEvent for WindowResizeEvent {}

impl Event for WindowResizeEvent {
    const EVENT_TYPE: EventType = EventType::WindowResize;
    const EVENT_CATEGORY: EventCategory = Self::WINDOW_EVENT_CATEGORY;
}

impl Debug for WindowResizeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}Event: {}, {}",
            Self::EVENT_TYPE,
            self.width,
            self.height
        )
    }
}

pub trait AppEvent {
    const APP_EVENT_CATEGORY: EventCategory = EventCategory::Application;
}

#[derive(Debug)]
pub struct AppTickEvent;

impl AppTickEvent {
    pub fn new() -> Self {
        Self
    }
}

impl AppEvent for AppTickEvent {}

impl Event for AppTickEvent {
    const EVENT_TYPE: EventType = EventType::AppTick;
    const EVENT_CATEGORY: EventCategory = Self::APP_EVENT_CATEGORY;
}

#[derive(Debug)]
pub struct AppUpdateEvent;

impl AppUpdateEvent {
    pub fn new() -> Self {
        Self
    }
}

impl AppEvent for AppUpdateEvent {}

impl Event for AppUpdateEvent {
    const EVENT_TYPE: EventType = EventType::AppUpdate;
    const EVENT_CATEGORY: EventCategory = Self::APP_EVENT_CATEGORY;
}

pub struct AppRenderEvent;

impl AppRenderEvent {
    pub fn new() -> Self {
        Self
    }
}

impl AppEvent for AppRenderEvent {}

impl Event for AppRenderEvent {
    const EVENT_TYPE: EventType = EventType::AppRender;
    const EVENT_CATEGORY: EventCategory = Self::APP_EVENT_CATEGORY;
}
