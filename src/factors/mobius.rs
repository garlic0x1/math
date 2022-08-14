use std::collections::HashSet;

use super::primes::*;

/// Möbius function μ(n)
pub fn mobius(n: u64) -> i8 {
    match unique_facts(n) {
        Ok(facts) => {
            if facts % 2 == 0 {
                1
            } else {
                -1
            }
        }
        Err(_) => 0,
    }
}

/// create an occurence map of prime factors
pub fn unique_facts(n: u64) -> Result<usize, ()> {
    let facts = prime_factors(n);
    let mut factmap = HashSet::new();
    for fact in facts {
        if !factmap.insert(fact) {
            return Err(());
        }
    }
    Ok(factmap.len())
}
