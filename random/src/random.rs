mod num;

use crate::num::Num;

pub struct Random<'a> {
    seed: &'a [u8],
    offset: u8,
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
impl<'a> Random<'a> {
    pub fn new(seed: &'a [u8]) -> Random<'a> {
        Self { seed, offset: 0 }
    }

    #[inline]
    fn slice(&mut self, size: usize) -> &[u8] {
        &self.seed[self.offset as usize..self.offset as usize + size]
    }

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

    /// Returns a random number in range [min, max).
    pub fn in_range<T>(&mut self, min: T, max: T) -> T where T: Num {
        if min < max {
            return min + self.roll(max - min);
        } else {
            return min;
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

    pub fn size(&mut self) -> u8 {
        return self.seed.len() as u8 - self.offset;
    }
}