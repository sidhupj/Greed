//main event module
mod event;

//sub event modules
mod application_events;
mod key_events;
mod mouse_events;

//extorting event mod entities
pub use event::Event;
pub use event::EventCategory;
pub use event::EventDispatcher;
pub use event::EventType;

//extporting key_events module entities
pub use key_events::KeyEvent;
pub use key_events::KeyPressedEvent;
pub use key_events::KeyReleasedEvent;

//extporting mouse_events module entities
pub use mouse_events::MouseButtonEvent;
pub use mouse_events::MouseButtonPressedEvent;
pub use mouse_events::MouseButtonReleasedEvent;
pub use mouse_events::MouseEvent;
pub use mouse_events::MouseMovedEvent;
pub use mouse_events::MouseScrolledEvent;

//extporting application_events module entities
pub use application_events::AppEvent;
pub use application_events::AppRenderEvent;
pub use application_events::AppTickEvent;
pub use application_events::AppUpdateEvent;
pub use application_events::WindowCloseEvent;
pub use application_events::WindowEvent;
pub use application_events::WindowResizeEvent;
