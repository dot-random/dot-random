pub struct Random {
    seed: u64,
}

/// Will have convenient methods to get a necessary size random number.
/// e.g. if your event happens with 0.15% chance you would do:
/// ```rust
///  if random.roll(1000) < 15 {
///     // hit
/// }
/// ```
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