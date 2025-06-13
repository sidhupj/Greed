use tracing::info;

pub trait App {
    fn new() -> Self;
    fn run(&self) {
        info!("App has started");
        println!("Running the application...");
        loop {}
    }
}
