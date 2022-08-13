use anyhow::{bail, Result};
use std::collections::HashMap;

pub struct BigFraction {
    pub numerator: HashMap<u32, u32>,
    pub denominator: HashMap<u32, u32>,
}

impl BigFraction {
    pub fn from_fact_map(numerator: HashMap<u32, u32>, denominator: HashMap<u32, u32>) -> Self {
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

    /// calculate to a u32 or fail on overflow
    pub fn numerator(&self) -> Result<u32> {
        let mut num: u32 = 1;
        for (prime, count) in self.numerator.iter() {
            if let Some(fact) = prime.checked_pow(*count) {
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

    /// calculate to a u32 or fail on overflow
    pub fn denominator(&self) -> Result<u32> {
        let mut den: u32 = 1;
        for (prime, count) in self.denominator.iter() {
            if let Some(fact) = prime.checked_pow(*count) {
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
