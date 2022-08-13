pub struct LEndian {
    base: u8,
    n: u64,
    place: u64,
}

impl LEndian {
    pub fn new(n: u64, base: u8) -> Self {
        Self { n, base, place: 1 }
    }
}

impl Iterator for LEndian {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.place <= self.n {
            let val = self.n % (self.place * self.base as u64) / self.place;
            self.place *= self.base as u64;
            Some(val as u8)
        } else {
            None
        }
    }
}
pub struct BEndian {
    base: u8,
    n: u64,
    place: u64,
}

impl BEndian {
    pub fn new(n: u64, base: u8) -> Self {
        let mut place = 1;
        while place <= n {
            place *= base as u64;
        }
        place /= base as u64;
        Self { n, base, place }
    }
}

impl Iterator for BEndian {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.place > 0 {
            let val = self.n % (self.place * self.base as u64) / self.place;
            self.place /= self.base as u64;
            Some(val as u8)
        } else {
            None
        }
    }
}
