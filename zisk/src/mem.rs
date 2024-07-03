use std::rc::Rc;

use wchelpers::{WCComponent, WCManager};

pub struct MemSM;

impl MemSM {
    const MY_NAME: &'static str = "MemSM    ";

    pub fn new(wcm: &mut WCManager) -> Rc<Self> {
        let mem_sm = Rc::new(MemSM);
        wcm.register_component(Rc::clone(&mem_sm) as Rc<dyn WCComponent>);

        mem_sm
    }
}

impl WCComponent for MemSM {
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
