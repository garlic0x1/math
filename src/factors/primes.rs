use std::collections::HashMap;

/// create an occurence map of prime factors
pub fn fact_map(n: u32) -> HashMap<u32, u32> {
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

pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    smallest_prime(n) == n
}
pub fn largest_prime(n: u32) -> u32 {
    let smallest = smallest_prime(n);
    if smallest == n {
        n
    } else {
        largest_prime(n / smallest)
    }
}
pub fn prime_factors(n: u32) -> Vec<u32> {
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
pub fn smallest_prime(n: u32) -> u32 {
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
        step = 6 - step;
    }

    n
}
