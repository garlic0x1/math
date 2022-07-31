use std::fmt::Debug;

use bitvec::{prelude::BitVec, *};

pub struct Binary {
    // stored little endian
    bitvec: BitVec,
}

impl Binary {
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
}
