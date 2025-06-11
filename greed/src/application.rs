use libloading::{Error, Library, Symbol};

type RunApplicationFn = unsafe extern "C" fn() -> i8;

pub struct Application {
    status: i8,
    engine_lib: Library,
    run_application_from_engine: RunApplicationFn,
}

impl Application {
    pub fn new() -> Result<Self, Error> {
        let lib = unsafe { Library::new("engine.dll")? };

        // Get the Symbol, then extract the raw function pointer.
        // The Symbol is temporary and dropped here, but the pointer is copied.
        let func_symbol: Symbol<RunApplicationFn> = unsafe { lib.get(b"run_application\0")? };
        let func_ptr = *func_symbol;
        Ok(Self {
            status: 0,
            engine_lib: lib,
            run_application_from_engine: func_ptr,
        })
    }

    pub fn run_application(&mut self) {
        self.status = 1;
        let new_status = unsafe { (self.run_application_from_engine)() };
        self.status = new_status;
    }
}
