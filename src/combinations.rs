use anyhow::{bail as yeet, Result};

use crate::{
    big_fraction::BigFraction,
    functions::{factorial_map, multiply_map},
};

pub fn n_combinations(n: u64, r: u64) -> Result<u64> {
    if n < r {
        yeet!("n < r, cannot make combinations");
    }

    let numerator = factorial_map(n);
    let mut denominator = factorial_map(r);
    let den2 = factorial_map(n - r);
    multiply_map(&mut denominator, &den2);
    let mut fraction = BigFraction::from_fact_map(numerator, denominator);
    fraction.simplify();

    let combinations = fraction.numerator()? / fraction.denominator()?;
    Ok(combinations)
}
