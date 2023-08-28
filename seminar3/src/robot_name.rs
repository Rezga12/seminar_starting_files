#[derive(Default)]
pub struct Robot;

impl Robot {
    pub fn new() -> Self {
        unimplemented!("Construct a new Robot struct.");
    }

    pub fn name(&self) -> &str {
        unimplemented!("Return a robot name.");
    }

    pub fn reset_name(&mut self) {
        unimplemented!("Reset a robot name to a new one.");
    }
}
