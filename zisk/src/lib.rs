mod main_sm;
mod mem;
mod zisk;

pub use main_sm::MainSM;
pub use mem::MemSM;
use wchelpers::WCLibrary;

pub use zisk::Zisk;

#[no_mangle]
pub extern "Rust" fn init_library<'a>() -> Box<dyn WCLibrary> {
    Box::new(Zisk::new())
}
