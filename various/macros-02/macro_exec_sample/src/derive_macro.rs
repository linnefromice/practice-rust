use derive_getter::Getter;

#[derive(Getter)]
pub struct Sample {
    pub field1: u64,
    pub field2: u64,
}