use anyhow::{bail as yeet, Result};
use std::collections::HashMap;

pub struct BigFraction {
    numerator: HashMap<u64, u32>,
    denominator: HashMap<u64, u32>,
}

impl BigFraction {
    pub fn from_fact_map(numerator: HashMap<u64, u32>, denominator: HashMap<u64, u32>) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub fn simplify(&mut self) {
        for (prime, num_count) in self.numerator.iter_mut() {
            if let Some(den_count) = self.denominator.get_mut(&prime) {
                while *num_count > 0 && *den_count > 0 {
                    *num_count -= 1;
                    *den_count -= 1;
                }
            }
        }
    }

    pub fn numerator(&self) -> u64 {
        let mut num = 1;
        for (prime, count) in self.numerator.iter() {
            num *= prime.pow(*count);
        }
        num
    }

    pub fn denominator(&self) -> u64 {
        let mut den = 1;
        for (prime, count) in self.denominator.iter() {
            den *= prime.pow(*count);
        }
        den
    }
}
