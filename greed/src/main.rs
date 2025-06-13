use engine::{app::App, log::Log};
use tracing::info;

struct GreedApp;
impl App for GreedApp {
    fn new() -> Self {
        GreedApp
    }
}
fn main() {
    // 2. Initialize the tracing subscriber
    // Store the returned Log struct in a variable that lives for the duration of main.
    let _log_setup = Log::init_non_blocking_console();
    info!("Logger has been initialized {}", 24);

    let greed_app = Box::new(GreedApp::new());
    greed_app.run();
    // The application will run indefinitely until manually stopped.
}
