use anyhow::{bail as yeet, Result};

pub fn n_combinations(n: u32, r: u32) -> Result<u32> {
    if n < r {
        yeet!("n < r, cannot make combinations");
    }

    Ok(1)
}
