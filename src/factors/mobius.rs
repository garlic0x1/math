use std::collections::HashSet;

use super::{primes::*, sieves::*};

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

/// returns the number of squarefrees up to n
pub fn squarefrees(n: u64) -> u64 {
    let limit = f64::sqrt(n as f64) as u64;

    let (factors, squares) = mobius_sieve(limit as usize);

    let mut count = 0;

    for i in 2..limit {
        // skip squares
        if squares[i as usize] {
            continue;
        }

        // find squares of the rest
        let square = i * i;
        let mults = n / square;

        // count -= mults * mu(i)
        if factors[i as usize] % 2 == 1 {
            // if i has an odd number, its mults are new
            count += mults;
        } else {
            // if i has an even number, we have seen its mults already
            count -= mults;
        }
    }

    n - count as u64
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
