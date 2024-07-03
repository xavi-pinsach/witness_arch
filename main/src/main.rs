use wchelpers::{Executor, WCLibrary};
use zisk::Zisk;

fn main() {
    println!("Hello, zisk!");

    let mut zisk = Zisk::new();

    zisk.start_proof();
    zisk.get_layout();
    zisk.calculate_witness();
    zisk.execute();
    zisk.end_proof();
}
