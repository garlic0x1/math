use anyhow::{bail, Result};
use std::collections::HashMap;

use crate::factors::primes::fact_map;

pub struct BigFraction {
    pub numerator: HashMap<u64, u64>,
    pub denominator: HashMap<u64, u64>,
}

impl BigFraction {
    pub fn new(numerator: u64, denominator: u64) -> Self {
        let numerator = fact_map(numerator);
        let denominator = fact_map(denominator);
        Self {
            numerator,
            denominator,
        }
    }

    pub fn from_fact_map(numerator: HashMap<u64, u64>, denominator: HashMap<u64, u64>) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    /// reduce the fraction
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

    /// calculate to a u64 or fail on overflow
    pub fn numerator(&self) -> Result<u64> {
        let mut num: u64 = 1;
        for (prime, count) in self.numerator.iter() {
            if let Some(fact) = prime.checked_pow(*count as u32) {
                if let Some(new) = num.checked_mul(fact) {
                    num = new;
                } else {
                    bail!("overflow");
                }
            } else {
                bail!("overflow");
            }
        }
        Ok(num)
    }

    /// calculate to a u64 or fail on overflow
    pub fn denominator(&self) -> Result<u64> {
        let mut den: u64 = 1;
        for (prime, count) in self.denominator.iter() {
            if let Some(fact) = prime.checked_pow(*count as u32) {
                if let Some(new) = den.checked_mul(fact) {
                    den = new;
                } else {
                    bail!("overflow");
                }
            } else {
                bail!("overflow");
            }
        }
        Ok(den)
    }
}
