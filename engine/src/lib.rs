use core;

pub struct Application {
    status: i8,
}

impl Application {
    pub fn new() -> Self {
        Self { status: 0 }
    }

    pub fn run(&mut self) {
        self.status = 1;
        println!("{}", self.status);
        println!("Greed Engine is running!");
        loop {}
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn run_application() -> i8 {
    let mut app = Application::new();
    app.run();
    app.status
}
