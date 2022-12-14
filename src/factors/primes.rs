use std::collections::HashMap;

pub fn smallest_prime(n: u64) -> u64 {
    if n % 2 == 0 {
        return 2;
    }

    if n % 3 == 0 {
        return 3;
    }

    let mut i = 5;
    let mut step = 2;

    while i * i <= n {
        if n % i == 0 {
            return i;
        }
        i += step;
        // magic
        step = 6 - step;
    }

    n
}

/// get the unique prime factors as a vector
pub fn prime_factors(n: u64) -> Vec<u64> {
    let mut facts = Vec::new();
    let smallest = smallest_prime(n);
    if smallest != n {
        facts.push(smallest);
        facts.extend(prime_factors(n / smallest));
    } else {
        facts.push(smallest);
    }
    facts
}

/// create an occurence map of prime factors
pub fn fact_map(n: u64) -> HashMap<u64, u64> {
    let facts = prime_factors(n);
    let mut factmap = HashMap::new();
    for fact in facts {
        if let Some(count) = factmap.get_mut(&fact) {
            *count += 1;
        } else {
            factmap.insert(fact, 1);
        }
    }
    factmap
}

pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    smallest_prime(n) == n
}

pub fn largest_prime(n: u64) -> u64 {
    let smallest = smallest_prime(n);
    if smallest == n {
        n
    } else {
        largest_prime(n / smallest)
    }
}
