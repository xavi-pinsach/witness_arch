pub trait WCLibrary {
    fn start_proof(&mut self);
    fn end_proof(&mut self);
    fn get_layout(&mut self);
    fn calculate_witness(&mut self);
}
