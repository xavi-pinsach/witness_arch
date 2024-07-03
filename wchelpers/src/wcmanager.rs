use std::rc::Rc;

use crate::{Executor, WCComponent};

pub struct WCManager {
    components: Vec<Rc<dyn WCComponent>>,
    executors: Vec<Rc<dyn Executor>>,
}

impl WCManager {
    const MY_NAME: &'static str = "WCManager";

    pub fn new() -> Self {
        WCManager {
            components: Vec::new(),
            executors: Vec::new(),
        }
    }

    pub fn register_component(&mut self, mem_sm: Rc<dyn WCComponent>) {
        self.components.push(mem_sm);
    }

    pub fn register_executor(&mut self, executor: Rc<dyn Executor>) {
        self.executors.push(executor);
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
        for executor in self.executors.iter() {
            executor.execute();
        }
    }
}
