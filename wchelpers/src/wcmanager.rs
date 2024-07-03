use std::rc::Rc;

use crate::WCComponent;

pub struct WCManager {
    components: Vec<Rc<dyn WCComponent>>,
    on_execute_fn: Option<Rc<dyn Fn() -> ()>>,
}

impl WCManager {
    const MY_NAME: &'static str = "WCManager";

    pub fn new() -> Self {
        WCManager {
            components: Vec::new(),
            on_execute_fn: None,
        }
    }

    pub fn register_component(&mut self, mem_sm: Rc<dyn WCComponent>) {
        self.components.push(mem_sm);
    }

    pub fn start_proof(&mut self) {
        println!("{}: Starting proof", Self::MY_NAME);
        for component in self.components.iter() {
            component.start_proof();
        }
    }

    pub fn end_proof(&mut self) {
        println!("{}: Ending proof", Self::MY_NAME);
        for component in self.components.iter() {
            component.end_proof();
        }
    }

    pub fn get_layout(&mut self) {
        println!("{}: Getting layout", Self::MY_NAME);
        for component in self.components.iter() {
            component.get_layout();
        }
    }

    pub fn calculate_witness(&mut self) {
        println!("{}: Calculating witness", Self::MY_NAME);
        for component in self.components.iter() {
            component.calculate_witness();
        }
    }

    pub fn execute(&self) {
        println!("{}: Executing", Self::MY_NAME);
        if let Some(ref f) = self.on_execute_fn {
            f();
        }
    }

    pub fn on_execute<F>(&mut self, f: F)
    where
        F: Fn() + 'static,
    {
        self.on_execute_fn = Some(Rc::new(f));
    }
}
