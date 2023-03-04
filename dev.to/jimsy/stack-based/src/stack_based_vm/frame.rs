pub struct Frame {
    pub return_address: usize
}

impl Frame {
    pub fn new(return_address: usize) -> Frame {
        Frame { return_address }
    }
}