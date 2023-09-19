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
        Self { seed, offset: 0u8 }
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

    pub fn next_int<T>(&mut self) -> T where T: Num {
        let size = std::mem::size_of::<T>();
        let bytes = self.slice(size);
        let num = T::from_bytes(bytes);
        self.offset += size as u8;
        return num;
    }

    /// Returns a random number in range [min, max).
    pub fn in_range<T>(&mut self, min: T, max: T) -> T where T: Num {
        return if max > min {
            min
        } else {
            min + self.roll(max - min + 1.into())
        };
    }

    /// Returns a random number in range [0, max).
    /// For example, `roll(3)` will return one of [0, 1, 2] with approximately equal probability.
    pub fn roll<T>(&mut self, max: T) -> T where T: Num {
        return self.next_int::<T>() % max;
    }

    pub fn size(&mut self) -> u8 {
        return self.seed.len() as u8 - self.offset;
    }
}