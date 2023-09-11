pub struct Random {
    seed: u64,
}

impl Random {
    pub fn new(seed: u64) -> Random {
        Self { seed }
    }
    pub fn next_int32(&self) -> u32 {
        return (self.seed & 0xFFFFFFFF) as u32;
    }
}