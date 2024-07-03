use std::rc::Rc;

use wchelpers::{Executor, WCLibrary, WCManager};

use crate::{MainSM, MemSM};

pub struct Zisk {
    pub wcm: WCManager,
    pub main_sm: Rc<MainSM>,
    pub mem_sm: Rc<MemSM>,
}

impl Zisk {
    pub fn new() -> Self {
        let mut wcm = WCManager::new();

        let mem_sm = MemSM::new(&mut wcm);
        let main_sm = MainSM::new(&mut wcm, &mem_sm);

        wcm.on_execute({
            let main_sm = Rc::clone(&main_sm);
            move || {
                main_sm.execute();
            }
        });

        Zisk {
            wcm,
            main_sm,
            mem_sm,
        }
    }
}

impl WCLibrary for Zisk {
    fn start_proof(&mut self) {
        self.wcm.start_proof();
    }

    fn end_proof(&mut self) {
        self.wcm.end_proof();
    }

    fn get_layout(&mut self) {
        self.wcm.get_layout();
    }

    fn calculate_witness(&mut self) {
        self.wcm.calculate_witness();
    }
}

impl Executor for Zisk {
    fn execute(&self) {
        self.main_sm.execute();
    }
}
