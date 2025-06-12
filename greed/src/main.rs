use engine::app::App;

struct GreedApp;
impl App for GreedApp {
    fn new() -> Self {
        GreedApp
    }
}
fn main() {
    let greed_app = Box::new(GreedApp::new());
    greed_app.run();
    // The application will run indefinitely until manually stopped.
}
