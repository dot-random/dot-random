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

    /**
     * Returns a random number in range [min, max).
     */
    pub fn in_range(&self, min: u32, max: u32) -> u32 {
        return (self.seed & 0xFFFFFFFF) as u32;
    }

    /**
     * Returns a random number in range [0, max).
     */
    pub fn roll(&self, max: u32) -> u32 {
        return self.in_range(0u32, max);
    }

}