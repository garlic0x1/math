use std::collections::HashMap;

pub fn fact_map(n: u64) -> HashMap<u64, u32> {
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
        step = 6 - step;
    }
    return n;
}

pub fn smallest_prime_sieve(n: u64, primes: &Vec<u64>) -> u64 {
    if primes.len() == 0 {
        return smallest_prime(n);
    }

    for prime in primes.iter() {
        if n % prime == 0 {
            return *prime;
        }
    }

    let mut i = primes.last().unwrap() + 2;

    while i * i <= n {
        if n % i == 0 {
            return i;
        }
        i += 2;
    }
    return n;
}

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

pub fn largest_prime(n: u64) -> u64 {
    let smallest = smallest_prime(n);
    if smallest == n {
        n
    } else {
        largest_prime(n / smallest)
    }
}

pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    smallest_prime(n) == n
}
