use tracing::{debug, error, info};

use crate::events::{Event, EventCategory, MouseButtonPressedEvent, WindowResizeEvent};

pub trait App {
    fn new() -> Self;
    fn run(&self) {
        info!("App has started");
        println!("Running the application...");

        let e = MouseButtonPressedEvent::new(0010);
        debug!("{:?}", e);

        if e.is_in_category(EventCategory::Application) {
            error!("Wrong category");
        }
        if e.is_in_category(EventCategory::Mouse) {
            info!("Correct category");
            debug!("{:?}", e);
        }
        if e.is_in_category(EventCategory::Input) {
            info!("Correct category");
            debug!("{:?}", e);
        }
        if e.is_in_category(EventCategory::MouseButton) {
            info!("Correct category");
            debug!("{:?}", e);
        }

        loop {}
    }
}
