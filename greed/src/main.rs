use crate::application::Application;

mod application;

fn main() {
    let result = Application::new();
    match result {
        Err(err) => panic!("Error: {}", err),
        Ok(mut app) => app.run_application(),
    }
}
