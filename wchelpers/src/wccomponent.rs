pub trait WCComponent {
    fn start_proof(&self);
    fn end_proof(&self);
    fn get_layout(&self);
    fn calculate_witness(&self);
}
