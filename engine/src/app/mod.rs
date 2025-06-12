pub trait App {
    fn new() -> Self;
    fn run(&self) {
        println!("Running the application...");
        loop {}
    }
}
