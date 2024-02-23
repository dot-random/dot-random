mod num;

use crate::num::Num;

pub struct Random {
    seed: Vec<u8>,
    offset: u8,
    rotations: u8,
}


/// Provides convenient methods to get a random number of a necessary size.
///
/// e.g. if your event happens with 0.15% chance you would do:
///
/// ```text
///  if random.roll(10000) < 15 {
///     // hit
///  }
/// ```
///
/// Or if you have 3 different outcomes with chances 0.1%, 9.9% and 90% respectively:
/// ```text
///  let dice = random.roll(1000);
///  if dice < 1 {
///    // A [0.1% chance]
///  } else if dice < 100 {
///    // B [A + B has 10% chance]
///  } else {
///    // C
///  }
/// ```
///
/// Or if you need a number in range (hatching period of 1000-2000 minutes, with average being `(1000+2000)/2=1500` ):
/// ```text
///  let period = random.in_range(1000, 2001);
/// ```
///
/// A 50/50 chance:
/// ```text
///  if random.next_bool() {
///    // A
///  } else {
///    // B
///  }
/// ```
impl Random {
    /// seed length must be a multiple of 4.
    pub fn new(seed: &Vec<u8>) -> Random {
        Self { seed: seed.clone(), offset: 0, rotations: 0 }
    }

    fn slice(&mut self, size: usize) -> &[u8] {
        let off = self.offset as usize;
        let end = off + size;
        if end <= self.seed.len() {
            return &self.seed[off..end];
        }

        self.rotate();
        return &self.seed[0..size];
    }

    // shifts the bits in the seed, so we can produce a new sequence of pseudorandom numbers.
    fn rotate(&mut self) {
        self.offset = 0;
        self.rotations += 1;

        let len = self.seed.len();
        let mut new_seed: Vec<u8> = Vec::with_capacity(len);
        // first, for each 4 byte tuple, apply Lehmer RNG to get a new 4 byte tuple
        let mut i = 0;
        while i < len {
            let bytes = &self.seed[i..i + 4];
            let state = Num::from_bytes(bytes);
            let new_state = Self::apply_lehmer_transition(state);
            new_seed.extend(new_state.to_le_bytes()); // flip the bytes from BE to LE
            i += 4;
        }
        self.seed = new_seed;

        // then, rotate by a prime number
        let k = if len == 4 { 3 } else if len == 8 { 5 } else { 11 };
        self.seed.rotate_right(k);
    }

    fn apply_lehmer_transition(state: u32) -> u32 {
        if state == 0 {
            // just a random prime number with at least 3 set bits in each byte
            // and all bytes different: [173, 22, 100, 37]
            return 627316397;
        }

        // We use Fermat number F5 instead of a prime because it conveniently just above u32 size,
        // while also produces a cycle of at least 640 pseudo-random numbers.
        // 2^32 + 1 = 641 x 6700417 = 4,294,967,297
        let modulo: u64 = 0xffffffff + 1;
        return ((state as u64) * 48271u64 % modulo) as u32;
    }

    /// Returns `true` or `false`.
    ///
    pub fn next_bool(&mut self) -> bool {
        let size = 1u8;
        let res = (self.slice(size as usize)[0] % 2) == 1;
        self.offset += size;
        return res;
    }

    /// Returns a random number of size T (u8, u16, u32, u64, usize).
    ///
    pub fn next_int<T>(&mut self) -> T where T: Num {
        let size = std::mem::size_of::<T>();
        let bytes = self.slice(size);
        let num = T::from_bytes(bytes);
        self.offset += size as u8;
        return num;
    }

    /// Returns a random number in the range [min, max).
    pub fn in_range<T>(&mut self, min: T, max: T) -> T where T: Num {
        return if min < max {
            min + self.roll(max - min)
        } else {
            min
        };
    }

    /// Returns a random number in range [0, max).
    /// For example, `roll(3)` will return one of [0, 1, 2] with approximately equal probability.
    pub fn roll<T>(&mut self, max: T) -> T where T: Num {
        assert!(max != 0.into(), "Max should be positive!");

        // no need to waste a random number on 1
        if max == 1.into() {
            return 0.into();
        }

        // special case for powers of 2
        if (max & (max - 1.into())) == 0.into() {
            return self.next_int::<T>() % max;
        }

        // general case - use only values inside the uniform distribution range
        let max_uniform: T = T::MAX - (<T as Into<T>>::into(T::MAX) % max);
        loop {
            let num: T = self.next_int::<T>();
            if num < max_uniform {
                return num % max;
            }
        }
    }

    /* For unit tests */
    pub fn size(&self) -> u8 {
        return self.seed.len() as u8 - self.offset;
    }
    pub fn rotations(&self) -> u8 {
        return self.rotations;
    }
    pub fn seed(&self) -> &Vec<u8> {
        return &self.seed;
    }
}