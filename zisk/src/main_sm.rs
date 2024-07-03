use std::rc::Rc;

use wchelpers::{Executor, WCComponent, WCManager};

use crate::MemSM;

#[allow(dead_code)]
pub struct MainSM {
    mem_sm: Rc<MemSM>,
}

impl MainSM {
    const MY_NAME: &'static str = "MainSM   ";

    pub fn new(wcm: &mut WCManager, mem_sm: &Rc<MemSM>) -> Rc<Self> {
        let main_sm = Rc::new(Self {
            mem_sm: Rc::clone(&mem_sm),
        });
        wcm.register_component(Rc::clone(&main_sm) as Rc<dyn WCComponent>);
        wcm.register_executor(Rc::clone(&main_sm) as Rc<dyn Executor>);
        main_sm
    }
}

impl WCComponent for MainSM {
    fn start_proof(&self) {
        println!("{}: Starting proof", Self::MY_NAME);
    }

    fn end_proof(&self) {
        println!("{}: Ending proof", Self::MY_NAME);
    }

    fn get_layout(&self) {
        println!("{}: Getting layout", Self::MY_NAME);
    }

    fn calculate_witness(&self) {
        println!("{}: Calculating witness", Self::MY_NAME);
    }
}

impl Executor for MainSM {
    fn execute(&self) {
        println!("{}: Executing", Self::MY_NAME);
    }
}
