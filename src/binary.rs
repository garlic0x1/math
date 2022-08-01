use anyhow::{bail as yeet, Result};
use bitvec::prelude::BitVec;

pub struct Binary {
    /// stored little endian
    bitvec: BitVec,
}

impl Binary {
    /// takes a big endian &str
    pub fn from_str(num: &str) -> Result<Self> {
        let mut bitvec = BitVec::new();
        for c in num.chars().rev() {
            match c {
                '1' => bitvec.push(true),
                '0' => bitvec.push(false),
                other => yeet!("{} is not a binary number", other),
            }
        }
        Ok(Self { bitvec })
    }

    pub fn from_u32(num: u32) -> Self {
        let mut bitvec = BitVec::new();
        let mut x = num;
        while x >= 1 {
            if x % 2 == 0 {
                bitvec.push(false);
            } else {
                bitvec.push(true);
            }

            x /= 2;
        }

        Self { bitvec }
    }

    pub fn to_u32(&self) -> u32 {
        let mut sum = 0;
        let mut place = 1;
        for bit in self.bitvec.iter() {
            if *bit {
                sum += place;
            }
            place *= 2
        }
        sum
    }

    pub fn little_endian(&self) -> String {
        let mut s = String::new();
        for bit in self.bitvec.iter() {
            if *bit {
                s.push('1');
            } else {
                s.push('0');
            }
        }
        s
    }

    pub fn big_endian(&self) -> String {
        self.little_endian().chars().rev().collect::<String>()
    }

    pub fn bitvec(&self) -> &BitVec {
        &self.bitvec
    }

    pub fn double(&mut self) {
        let mut bitvec = BitVec::new();
        bitvec.push(false);
        bitvec.extend(self.bitvec.clone());
        self.bitvec = bitvec;
    }

    pub fn even(&self) -> bool {
        if let Some(b) = self.bitvec.first() {
            *b == false
        } else {
            true
        }
    }
}
